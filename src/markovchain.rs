use std::collections::BTreeMap;
use json;

struct MarkovChainRow
{
    column_map: BTreeMap<char, u64>,
    sum : u64
}

#[allow(dead_code)]
impl MarkovChainRow
{
    pub fn new() -> MarkovChainRow
    {
        MarkovChainRow { column_map: BTreeMap::new(), sum: 0 }
    }

    pub fn get_value(self: &Self, col: &char) -> u64
    {
        let mut result : u64 = 0;

        match self.column_map.get(&col)
        {
            Some(value) => result = value.clone(),
            None => {}
        }

        result
    }

    pub fn get_sum(self: &Self) -> u64
    {
        self.sum
    }

    pub fn inc_value(self: &mut Self, col: &char)
    {
        match self.column_map.get_mut(col)
        {
            Some(value) => *value += 1,
            None =>  { 
                self.column_map.insert(*col, 1); 
            }
        }
        self.sum += 1;
    }

    pub fn to_json_value(self: &Self) -> Box<json::JsonValue>
    {
        let mut json_object = json::JsonValue::new_object();

        json_object["sum"] = self.sum.into();
        json_object["values"] = json::JsonValue::new_object();
        
        for (col_char, value) in &self.column_map
        {
            json_object["values"][col_char.to_string()] = value.to_string().into();
        }

        Box::new(json_object)
    }
}

pub struct MarkovChain
{
    row_map: BTreeMap<char, MarkovChainRow>,
    last_value : char,
    case_sensitive : bool
}

impl MarkovChain
{
    pub fn new() -> MarkovChain
    {
        MarkovChain { 
            row_map: BTreeMap::new(),
            last_value: ' ',
            case_sensitive: false
        }
    }

    pub fn feed_character(self: &mut Self, value : &char)
    {
        let mut modified_value = *value;

        if !self.case_sensitive
        {
            modified_value = (*value).to_ascii_lowercase();
        }

        if !self.row_map.contains_key(&self.last_value)
        {
            let mut row = MarkovChainRow::new();
            row.inc_value(&modified_value);
            self.row_map.insert(self.last_value.clone(), row);
        }
        else
        {
            match self.row_map.get_mut(&self.last_value)
            {
                Some(row) => row.inc_value(&modified_value),
                None => assert!(false, "Unable to get mutable BTreeMap entry")
            }
        }

        self.last_value = modified_value;
    }

    pub fn feed_string(self: &mut Self, value : String)
    {
        for c in value.chars()
        {
            self.feed_character(&c);
        }
    }

    pub fn to_json(self: &Self, minimize : bool) -> Box<String>
    {
        let mut json_object = json::JsonValue::new_object();

        for (row_char, row) in &self.row_map
        {
           json_object[row_char.to_string()] = *row.to_json_value();
        }

        let result = if minimize { json_object.dump() } else { json::stringify_pretty(json_object, 2) };
                
        Box::new(result)
    }
}