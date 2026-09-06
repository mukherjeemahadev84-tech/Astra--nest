use chrono::{DateTime, Utc};

/// Decides whether a user (patient or doctor) currently has full access —
/// either because their 30-day free trial hasn't ended yet, or because they've
/// converted to a paid plan (is_pro / is_paying).
///
/// Used before returning Pro-only data (trend history, AI chat, doctor dashboard)
/// so a lapsed trial with no payment falls back to the free-tier response.
pub fn has_full_access(trial_ends_at: DateTime<Utc>, is_paid: bool) -> bool {
    is_paid || Utc::now() < trial_ends_at
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn trial_still_active_grants_access() {
        let future = Utc::now() + Duration::days(5);
        assert!(has_full_access(future, false));
    }

    #[test]
    fn expired_trial_without_payment_denies_access() {
        let past = Utc::now() - Duration::days(1);
        assert!(!has_full_access(past, false));
    }

    #[test]
    fn expired_trial_with_payment_still_grants_access() {
        let past = Utc::now() - Duration::days(1);
        assert!(has_full_access(past, true));
    }
}
