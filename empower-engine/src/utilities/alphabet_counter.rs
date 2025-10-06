pub struct AlphabetCounter
{
    current_letter: usize,
    alphabet: &'static str,
}

impl AlphabetCounter
{
    pub fn new() -> Self
    {
        Self 
        { 
            current_letter: 0, 
            alphabet: "abcdefghijklmnopqrstuwxyz", 
        }
    }

    pub fn next_letter(&mut self) -> char
    {
        // @TODO, modify this function to return aa, ab, ba and so on when exceding the initial characters

        let chars_array: Vec<char> = self.alphabet.chars().collect();

        let char_to_return = chars_array[self.current_letter];

        if chars_array.len() - 1 > self.current_letter
        {
            self.current_letter += 1;
        }

        return char_to_return;
    }
}