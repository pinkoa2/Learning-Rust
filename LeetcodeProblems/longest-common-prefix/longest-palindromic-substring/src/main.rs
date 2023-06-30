use std::collections::HashMap;

// Determines if the string is a palindrom or not
fn is_palindrom(s: &String) -> bool {
    let reversed: String = s.chars().rev().collect();
    return reversed == s.to_string();
}

// Gets the 2 largest substrings of a String
fn get_both_substrings(s: &String) -> (String, String) {
    return (s[1..].to_string(), s[0..s.len() - 1].to_string());
}

fn solve_problem_recursively(s: &String, map: &mut HashMap<String, String>) -> String {
    // println!("{s}");
    match map.get(s) {
        Some(palindrom) => return palindrom.to_string(),
        None => (),
    };

    if is_palindrom(s) {
        map.insert(s.to_string(), s.to_string());
        return s.to_string();
    }

    let (string1, string2) = get_both_substrings(s);

    let palindrom_of_string1 = solve_problem_recursively(&string1, map);
    let palindrom_of_string2 = solve_problem_recursively(&string2, map);

    if palindrom_of_string1.len() > palindrom_of_string2.len() {
        map.insert(s.to_string(), palindrom_of_string1);
    } else {
        map.insert(s.to_string(), palindrom_of_string2);
    }

    match map.get(s) {
        Some(palindrom) => return palindrom.to_string(),
        None => return "".to_string(),
    };
}

fn longest_palindrom(s: String) -> String {
    let mut map: HashMap<String, String> = HashMap::new();
    solve_problem_recursively(&s, &mut map);
    match map.get(&s) {
        Some(result) => return result.to_string(),
        None => return "".to_string(),
    };
}

fn main() {
    let input: String = String::from("ibvjkmpyzsifuxcabqqpahjdeuzaybqsrsmbfplxycsafogotliyvhxjtkrbzqxlyfwujzhkdafhebvsdhkkdbhlhmaoxmbkqiwiusngkbdhlvxdyvnjrzvxmukvdfobzlmvnbnilnsyrgoygfdzjlymhprcpxsnxpcafctikxxybcusgjwmfklkffehbvlhvxfiddznwumxosomfbgxoruoqrhezgsgidgcfzbtdftjxeahriirqgxbhicoxavquhbkaomrroghdnfkknyigsluqebaqrtcwgmlnvmxoagisdmsokeznjsnwpxygjjptvyjjkbmkxvlivinmpnpxgmmorkasebngirckqcawgevljplkkgextudqaodwqmfljljhrujoerycoojwwgtklypicgkyaboqjfivbeqdlonxeidgxsyzugkntoevwfuxovazcyayvwbcqswzhytlmtmrtwpikgacnpkbwgfmpavzyjoxughwhvlsxsgttbcyrlkaarngeoaldsdtjncivhcfsaohmdhgbwkuemcembmlwbwquxfaiukoqvzmgoeppieztdacvwngbkcxknbytvztodbfnjhbtwpjlzuajnlzfmmujhcggpdcwdquutdiubgcvnxvgspmfumeqrofewynizvynavjzkbpkuxxvkjujectdyfwygnfsukvzflcuxxzvxzravzznpxttduajhbsyiywpqunnarabcroljwcbdydagachbobkcvudkoddldaucwruobfylfhyvjuynjrosxczgjwudpxaqwnboxgxybnngxxhibesiaxkicinikzzmonftqkcudlzfzutplbycejmkpxcygsafzkgudy");

    let result = longest_palindrom(input);

    println!("{result}");
}

/*


Given a string s, return the longest
palindromic

substring
 in s.



Example 1:

Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.
Example 2:

Input: s = "cbbd"
Output: "bb"


Constraints:

1 <= s.length <= 1000
s consist of only digits and English letters.



babad
    -> baba
        -> bab
            X
            -> ba
                -> b
                -> a
            -> ab
                -> a
                -> b
        -> aba
            X
    -> abad
        -> aba
            X
        -> bad
            X


Get the longest palindrome
    - if a palindrome -> return yourself

    Get 2 substrings
        - get the longest palindrome for s1
        - get the longest palindrome for s2

    Optimizations:
        - keep track of length of longest, if we even try to get length of a string that is smaller STOP
        - some kind of dynamic programming by keeping a map of already tested strings
*/
