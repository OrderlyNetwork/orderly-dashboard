use crate::service_base::sdk::solana::{
    option_serializer::OptionSerializer,
    transaction_status::EncodedConfirmedTransactionWithStatusMeta,
};

pub fn decode_confirm_transaction_log_messages(
    tx: EncodedConfirmedTransactionWithStatusMeta,
) -> Vec<String> {
    if let Some(meta) = tx.transaction.meta {
        if let OptionSerializer::Some(log_messages) = meta.log_messages {
            return log_messages;
        }
    }
    vec![]
}

pub fn split_program_from_log(log: &str) -> Option<String> {
    let splited: Vec<&str> = log.split(' ').collect();
    if splited.len() != 4 {
        return None;
    }
    if splited[0] == "Program" && splited[2] == "invoke" {
        return Some(splited[1].to_string());
    }
    None
}

pub fn split_program_exit_log(log: &str) -> Option<String> {
    let splited: Vec<&str> = log.split(' ').collect();
    if splited.len() < 3 {
        return None;
    }
    if splited[0] == "Program" && splited[2] == "success" || splited[2] == "failed:" {
        return Some(splited[1].to_string());
    }
    None
}

pub fn split_program_event(log: &str) -> Option<String> {
    let splited: Vec<&str> = log.split(' ').collect();
    if splited.len() != 3 {
        return None;
    }
    if splited[0] == "Program" && splited[1] == "data:" {
        return Some(splited[2].to_string());
    }
    None
}

#[cfg(test)]
pub mod tests {
    use super::split_program_exit_log;

    #[test]
    fn test_split_program_exit_log() {
        let log = "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success";
        let program = split_program_exit_log(log);
        println!("exit program: {:?}", program);
        assert_eq!(
            program,
            Some("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string())
        );
    }

    #[test]
    fn test_failed_split_program_exit_log() {
        let log = "Program EK2EN13jdKj286QmVnipwYANjwWZJZM74mwDvyfuFaW6 failed: custom program error: 0x1770";
        let program = split_program_exit_log(log);
        println!("exit program: {:?}", program);
        assert_eq!(
            program,
            Some("EK2EN13jdKj286QmVnipwYANjwWZJZM74mwDvyfuFaW6".to_string())
        );
    }
}
