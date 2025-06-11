mod sanity {
    use crate::models::{
        ExtensionId, Notification, NotificationPayload, PaymentId, QRId, Signature, SignatureKey,
    };

    fn predefined_notification() -> Notification {
        return Notification {
            amount: 0.into(),
            commission: 0.into(),
            currency: crate::models::Currency::MDL,
            executed_at: "2029-10-22T10:32:28+03:00".to_owned(),
            extension_id: ExtensionId::new("extension_id".to_owned()),
            order_id: None,
            pay_id: PaymentId::new("pay_id".to_owned()),
            payer_iban: "payer_iban".to_owned(),
            payer_name: "payer_name".to_owned(),
            qr_id: QRId::new("qr_id".to_owned()),
            qr_status: crate::models::QRStatus::Paid,
            reference_id: "reference_id".to_owned(),
            terminal_id: None,
        };
    }

    #[test]
    fn validate_signature() {
        let notification = predefined_notification();
        let signature_key = SignatureKey::from("foobar".to_owned());
        let signature = Signature::new("NTFkNzc3ZmZlZjg0MjU0N2I4ODEzYzhmNjQ0N2ZkN2IzODY4Zjk2NGUwZjliMDAxODI5NmFlNDU1N2EyMDdmZA==".to_owned());

        let payload = NotificationPayload {
            result: notification,
            signature: signature.clone(),
        };

        let sig = payload.build_signature(signature_key);

        assert_eq!(sig, signature);
    }

    #[test]
    fn validate_signature_with_order_id() {
        let mut notification = predefined_notification();
        notification.order_id = Some("order_id".to_owned());

        let signature_key = SignatureKey::from("foobar".to_owned());
        let signature = Signature::new("NmFjNTczNGM3YzVjMGZhNzE1Nzk4NThiMDY2ZGQ3NDkwMDViNTQ2YzkzNWI4ZDUzNjAwYjA3ZTYwOTZiZjVlNg==".to_owned());

        let payload = NotificationPayload {
            result: notification,
            signature: signature.clone(),
        };

        let sig = payload.build_signature(signature_key);

        assert_eq!(sig, signature);
    }

    #[test]
    fn validate_signature_with_terminal_id() {
        let mut notification = predefined_notification();
        notification.terminal_id = Some("terminal_id".to_owned());

        let signature_key = SignatureKey::from("foobar".to_owned());
        let signature = Signature::new("MTYzYTI1Y2FiYjFkNzVlMTlhMjg5MTc4YTkxOTJlMzEyMGRkYTg2NjhmOGI3OWE1N2VjYWEwMTgxYWU3MWNlZg==".to_owned());

        let payload = NotificationPayload {
            result: notification,
            signature: signature.clone(),
        };

        let sig = payload.build_signature(signature_key);

        assert_eq!(sig, signature);
    }

    #[test]
    fn validate_signature_with_order_and_terminal_id() {
        let mut notification = predefined_notification();
        notification.order_id = Some("order_id".to_owned());
        notification.terminal_id = Some("terminal_id".to_owned());

        let signature_key = SignatureKey::from("foobar".to_owned());
        let signature = Signature::new("MDMyNGNiOTYwN2Y2NzZjYmY5MDJkMjhlYTgwMzRkODU0NjdhMmUzZjA2MTU3NGNhYjNhNTBiNDk4NWFkZTczYw==".to_owned());

        let payload = NotificationPayload {
            result: notification,
            signature: signature.clone(),
        };

        let sig = payload.build_signature(signature_key);

        assert_eq!(sig, signature);
    }
}
