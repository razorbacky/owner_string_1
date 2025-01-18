// Ownership String #1 - 25.01.19
// 소유권

fn main() {
    // let s = String::from("Hello");
    // 이중 콜론 :: 은 함수를 사용할 때, string_from 같은 함수명을 사용하지 않고, string 타입에 있는 특정된 from 함수라는 것을 지정할 수 있게 해주는 네임스페이스 연산자이다.

    let mut s = String::from("Hello");

    s.push_str(", world!"); // push_str()이 String 에 리터럴을 추가한다.

    // drop(s); drop() 함수를 추가하여 사용자가 직접 해제할 수도 있다.

    println!("{}", s); // 이 줄이 'hello, world!' 를 출력한다.

    /*
    Rust가  메모리를 해제하는 방식은 Scope를 벗어나는 순간 자동으로 메모리를 해제하는 방식으로 해결함
    {
        let s = Stinrg::from("Hello"); // s 라는 변수는 이 지점부터 유효하다.

        // s 를 가지고 어떤 작업을 한다.

    } // 이 Scope가 종료되었고, s 는 더 이상 유효하지 않다.
    // Scope가 끝나는 지점에 자동으로 drop() 함수가 호출되는 방식이다.

    NOTE : C++ 에서는 이런 식으로 아이템의 수명이 끝나는 시점에 리소스를 해제하는 패턴을 Resource Acquisition Is Initialization(RAII) 라고 한다.
    */
}
