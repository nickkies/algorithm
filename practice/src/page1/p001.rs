mod data_structure {
    /// 1부터 8까지 차례대로 라면 ascending,
    /// 8부터 1까지 차례대로 라면 descending,
    /// 둘 다 아니라면 mixed 이다.
    /// 순서가 주어졌을 때,
    /// 이것이 ascending인지, descending인지, 아니면 mixed인지 판별하는 프로그램을 작성하시오.
    #[test]
    fn test_array_solution() {
        let (input1, output1) = ([1, 2, 3, 4, 5, 6, 7, 8], "ascending".to_string());
        let (input2, output2) = ([8, 7, 6, 5, 4, 3, 2, 1], "descending".to_string());
        let (input3, output3) = ([8, 1, 7, 2, 6, 3, 4, 5], "mixed".to_string());

        assert_eq!(array_solution(input1), output1);
        assert_eq!(array_solution(input2), output2);
        assert_eq!(array_solution(input3), output3);
    }

    fn array_solution(arr: [i8; 8]) -> String {
        let mut direction = 0;

        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];

            if direction == 0 {
                direction = diff;
            } else if direction * diff != 1 {
                return "mixed".to_string();
            }
        }

        if direction > 0 {
            "ascending".to_string()
        } else {
            "descending".to_string()
        }
    }
}
