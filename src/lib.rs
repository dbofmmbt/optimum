pub mod stop_criterion;
pub mod percentage;
pub mod elite_set;
pub mod coverage;
pub mod selection_control;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
