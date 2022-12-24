pub struct Tsv;

impl Tsv
{
    pub fn break_down_line(line:&String) -> (String,String,String)
    {


        if line.to_string()
        .is_empty()
        {
            return (" ".to_string(), " ".to_string(), " ".to_string());
        }

        let code:String;
        let name:String;
        let mut seq: String;

        {
            let string:String = line.to_string();
            let vect:Vec<&str> = string.split_whitespace()
            .collect();
            code = vect[0].trim().to_string();
        }
        {
            let string:String = line.to_string();
            let vect:Vec<&str> = string.split("1.").collect();

            name = vect[0].trim()
            .to_string()
            .replace(&code,"");

            seq = String::from("1.");
            seq.push_str(&vect[1]
            .to_string());

        }


        (code, name, seq
        .trim()
        .to_string())

    }

    
}