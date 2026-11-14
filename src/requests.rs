//! Which gamma state to ask the worker for. Pure: no I/O; the caller sends
//! the targets these methods return and reports each reply.
//!
//! At most one request is in flight. Input that arrives meanwhile is never
//! dropped: it replaces the wanted target, and the reply sends the latest
//! one (so a slider drag costs one round trip per reply, not one per step).

/// What the screen should show: `Some(kelvin)` tinted, `None` neutral.
pub type Target = Option<u32>;

#[derive(Debug, Default)]
pub struct Requests {
    /// The latest target asked for; the popup and icon show this.
    wanted: Target,
    /// The target sent and not answered yet.
    in_flight: Option<Target>,
    /// The target the backend confirmed last.
    applied: Target,
}

/// What to do after a reply.
#[derive(Debug, PartialEq)]
pub struct Replied {
    /// The target the reply answered (`None` for a reply nothing waited for).
    pub sent: Option<Target>,
    /// The next target to send.
    pub send: Option<Target>,
}

impl Requests {
    /// Night light is wanted on (shown at once, before the backend answers).
    pub fn is_on(&self) -> bool {
        self.wanted.is_some()
    }

    /// Ask for `target`. Returns the target to send now, if any: nothing is
    /// sent while a request is in flight (its reply sends the latest target)
    /// or when the backend already shows `target`.
    pub fn want(&mut self, target: Target) -> Option<Target> {
        self.wanted = target;
        self.next()
    }

    /// The worker answered the request in flight. A success records the
    /// target as applied. A failed enable leaves gamma neutral (the worker
    /// resets it); a failed disable leaves it as it was. If nothing newer was
    /// asked for meanwhile, the wanted state falls back to the real one, so
    /// the UI shows reality and a failure is not retried in a loop.
    pub fn replied(&mut self, ok: bool) -> Replied {
        let sent = self.in_flight.take();
        if let Some(target) = sent {
            if ok {
                self.applied = target;
            } else {
                if target.is_some() {
                    self.applied = None;
                }
                if self.wanted == target {
                    self.wanted = self.applied;
                }
            }
        }
        Replied {
            sent,
            send: self.next(),
        }
    }

    fn next(&mut self) -> Option<Target> {
        if self.in_flight.is_some() || self.wanted == self.applied {
            return None;
        }
        self.in_flight = Some(self.wanted);
        Some(self.wanted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_off_and_sends_a_change_at_once() {
        let mut r = Requests::default();
        assert!(!r.is_on());
        assert_eq!(r.want(None), None);
        assert_eq!(r.want(Some(3500)), Some(Some(3500)));
        assert!(r.is_on(), "shown on before the reply");
    }

    #[test]
    fn toggle_while_in_flight_is_kept_and_sent_on_reply() {
        let mut r = Requests::default();
        assert_eq!(r.want(Some(3500)), Some(Some(3500)));
        // Toggled off again before the worker answered: shown at once.
        assert_eq!(r.want(None), None);
        assert!(!r.is_on());
        assert_eq!(
            r.replied(true),
            Replied {
                sent: Some(Some(3500)),
                send: Some(None)
            }
        );
        assert_eq!(
            r.replied(true),
            Replied {
                sent: Some(None),
                send: None
            }
        );
        assert!(!r.is_on());
    }

    #[test]
    fn slider_drag_coalesces_to_the_latest_value() {
        let mut r = Requests::default();
        assert_eq!(r.want(Some(3500)), Some(Some(3500)));
        for k in (2000..3500).step_by(100) {
            assert_eq!(r.want(Some(k)), None);
        }
        assert_eq!(r.want(Some(2100)), None);
        assert_eq!(r.replied(true).send, Some(Some(2100)));
        assert_eq!(r.replied(true).send, None);
    }

    #[test]
    fn same_target_as_applied_sends_nothing() {
        let mut r = Requests::default();
        r.want(Some(3500));
        r.replied(true);
        assert_eq!(r.want(Some(3500)), None);
        // Back to the applied value while a request is in flight: the reply
        // sends it again, since the backend now shows the other value.
        assert_eq!(r.want(Some(3000)), Some(Some(3000)));
        assert_eq!(r.want(Some(3500)), None);
        assert_eq!(r.replied(true).send, Some(Some(3500)));
    }

    #[test]
    fn failed_enable_shows_off_and_does_not_retry() {
        let mut r = Requests::default();
        r.want(Some(3500));
        assert_eq!(
            r.replied(false),
            Replied {
                sent: Some(Some(3500)),
                send: None
            }
        );
        assert!(!r.is_on());
        // The next press tries again.
        assert_eq!(r.want(Some(3500)), Some(Some(3500)));
    }

    #[test]
    fn failed_disable_keeps_it_on() {
        let mut r = Requests::default();
        r.want(Some(3500));
        r.replied(true);
        assert_eq!(r.want(None), Some(None));
        assert_eq!(r.replied(false).send, None);
        assert!(r.is_on());
    }

    #[test]
    fn failure_with_newer_input_sends_the_newer_target_once() {
        let mut r = Requests::default();
        r.want(Some(3500));
        r.want(Some(3000));
        // The enable failed (gamma is neutral now); 3000 K is still wanted.
        assert_eq!(r.replied(false).send, Some(Some(3000)));
        assert_eq!(r.replied(false).send, None);
        assert!(!r.is_on());
    }

    #[test]
    fn reply_with_nothing_in_flight_is_ignored() {
        let mut r = Requests::default();
        assert_eq!(
            r.replied(true),
            Replied {
                sent: None,
                send: None
            }
        );
    }
}
