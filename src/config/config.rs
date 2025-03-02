use crate::library::RuleType;

use serde::de::{Deserializer, Error};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    #[serde(rename = "ValidationRules")]
    pub validation_rules: ValidationRulesContainer,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ValidationRulesContainer {
    #[serde(rename = "Groups")]
    pub groups: GroupsContainer,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GroupsContainer {
    #[serde(rename = "ValidationRulesGroup")]
    pub validation_rules_groups: Vec<ValidationRulesGroupSettings>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ValidationRulesGroupSettings {
    #[serde(rename = "@Group")]
    pub group: String,
    #[serde(rename = "ValidationRule")]
    pub validation_rules: Vec<ValidationRuleSettings>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ValidationRuleSettings {
    #[serde(rename = "@Type")]
    pub rule_type: RuleType,
    #[serde(rename = "@Enabled", deserialize_with = "parse_bool")]
    pub enabled: bool,
    #[serde(rename = "@FallbackShiftStatusId", default)]
    pub fallback_shift_status_id: Option<i32>,
    #[serde(
        rename = "@PositionTypeIDs",
        deserialize_with = "parse_csv_string",
        default
    )]
    pub position_type_ids: Vec<i32>,
    #[serde(rename = "@FromMatchStatusId", default)]
    pub from_match_status_id: Option<i32>,
    #[serde(rename = "Rules")]
    pub rules: RulesContainer,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct RulesContainer {
    #[serde(rename = "Rule", default)]
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Rule {
    #[serde(
        rename = "@ForCandidateStatusIds",
        deserialize_with = "parse_csv_string",
        default
    )]
    for_candidate_status_ids: Vec<i32>,
    #[serde(rename = "@Enforce", deserialize_with = "parse_bool", default)]
    enforce: bool,
}

fn parse_csv_string<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().map_err(Error::custom))
        .collect()
}

fn parse_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        "" => Ok(false),
        " " => Ok(false),
        _ => Err(D::Error::custom(format!(
            "Invalid [THIS IS WHERE WE NEED TO PROPAGATE OUR CUSTOM ERR] >> boolean value: {}",
            s
        ))),
    }
}

#[derive(Debug)]
pub struct Location {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}
impl std::error::Error for Location {}

//cargo test test_cfg_parsing -- --nocapture
#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use std::io::Cursor;
    use std::str;

    #[test]
    fn test_simple() {
        let xml = r#"<tag1 att1 = "test">
                    <tag2><!--Test comment-->Test</tag2>
                    <tag3>Test 2</tag3>
                </tag1>"#;
        let mut reader = Reader::from_reader(Cursor::new(xml.as_bytes()));
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"tag1" | b"tag2" => (),
                    tag => {
                        assert_eq!(b"tag3", tag);
                        // assert_eq!((3, 27), into_line_and_column(reader)); //marks ending of < 'tag3' > in this file
                        break;
                    }
                },
                Ok(Event::Eof) => unreachable!(),
                _ => (),
            }
            buf.clear();
        }
    }

    /// Find the byte position of the search string
    fn find_position_of_string(content: &str, search_string: &str) -> Option<(usize, usize)> {
        if let Some(byte_pos) = content.find(search_string) {
            let prefix = &content[0..byte_pos];

            let lines: Vec<&str> = prefix.split('\n').collect();
            let line = lines.len();
            let column = if lines.is_empty() {
                1
            } else {
                lines.last().unwrap().chars().count() + 1 //columns 1 indexed
            };

            return Some((line, column));
        }

        None
    }

    #[test]
    fn test_cfg_parsing() {
        let xml = r#"
<config>
    <!--
        Rules Config ======================
        This config is parsed and used as 'Rule validation engine'
    -->
    <ValidationRules>
      <Groups>
        <ValidationRulesGroup Group="Candidate">
          <ValidationRule Type="IndecisivePrevention" Enabled="True">
            <Rules>
              <Rule ForCandidateStatusIds="" IfShiftEndReasonIds="1" Enforce="False" ForTheNextXDays=""/>
            </Rules>
          </ValidationRule>
        </ValidationRulesGroup>
        <ValidationRulesGroup Group="Shift">
          <ValidationRule Type="SideJobPrevention" PositionTypeIDs="a" FromMatchStatusId="c" Enabled="True">
            <Rules>
              <Rule ForCandidateStatusIds="" Enforce="true"/>
            </Rules>
          </ValidationRule>
          <ValidationRule Type="LastMinuteActionPreventionForBooking" Enabled="True">
            <Rules>
              <Rule ForCandidateStatusIds="" Minutes="" Enforce="false"/>
            </Rules>
          </ValidationRule>
          <ValidationRule Type="LastMinuteActionPreventionForCanceling" Enabled="False">
            <Rules>
              <Rule ForCandidateStatusIds="" Minutes="" Enforce="False"/>
            </Rules>
          </ValidationRule>
          <ValidationRule Type="ExhaustionPrevention" FallbackShiftStatusId="1" Enabled="False">
            <Rules>
              <Rule ForCandidateStatusIds="" ForShiftStatusIds="" HoursAllowed="" WithinXHours="" Enforce="False"/>
            </Rules>
          </ValidationRule>
        </ValidationRulesGroup>
      </Groups>
    </ValidationRules>
  </config>
"#;
        let mut reader = Reader::from_reader(Cursor::new(xml.as_bytes()));
        //reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                //All attributes other than
                Ok(Event::Start(ref e)) => {
                    println!(
                        "Start tag: {:?}\n",
                        str::from_utf8(e.name().as_ref()).unwrap()
                    );

                    // Validate attributes
                    for attr in e.attributes() {
                        let attr_key =
                            String::from_utf8(attr.as_ref().unwrap().key.0.to_vec()).unwrap();
                        let attr_val =
                            String::from_utf8(attr.as_ref().unwrap().value.to_vec()).unwrap();

                        dbg!(attr_key, attr_val);

                        if let Ok(attr) = attr {
                            if attr.key.as_ref() == b"PositionTypeIDs" {
                                if is_num_sequence(&attr) {
                                    continue;
                                }

                                let position = reader.buffer_position() as usize;
                                let buffer = reader.get_ref().get_ref(); //get ref to underlying Cursor ->  &[u8]
                                let content =
                                    String::from_utf8(buffer[0..position].to_vec()).unwrap();
                                if let Some((line, column)) =
                                    find_position_of_string(&content, "PositionTypeIDs")
                                {
                                    eprintln!(
                                        "#Found 'PositionTypeIDs' at line {} column {} : \n{:?}\n",
                                        line, column, e
                                    );
                                    //TOOD : custom erorr appended to list
                                    // errors.push(Err(quick_xml::Error::InvalidAttr(
                                    //     quick_xml::events::attributes::AttrError::ExpectedValue(byte_position)
                                    // )));
                                }
                            }
                            if attr.key.as_ref() == b"FromMatchStatusId" {
                                if is_num(&attr) {
                                    continue;
                                }

                                let position = reader.buffer_position() as usize;
                                let buffer = reader.get_ref().get_ref(); //get ref to underlying Cursor ->  &[u8]
                                let content =
                                    String::from_utf8(buffer[0..position].to_vec()).unwrap();

                                if let Some((line, column)) =
                                    find_position_of_string(&content, "FromMatchStatusId")
                                {
                                    eprintln!(
                                        "#Found 'FromMatchStatusId' at line {}, column {}: \n{:?}\n",
                                        line, column, e
                                    );
                                }
                            }
                        }
                    }
                }
                //All Self closing <Rule /> tags
                Ok(Event::Empty(ref e)) => {
                    println!(
                        "Empty tag: {:?}\n",
                        str::from_utf8(e.name().as_ref()).unwrap()
                    );

                    for attr in e.attributes() {
                        let a = &attr;
                        let attr_key =
                            String::from_utf8(a.as_ref().unwrap().key.0.to_vec()).unwrap();
                        let attr_val =
                            String::from_utf8(a.as_ref().unwrap().value.to_vec()).unwrap();

                        dbg!(attr_key, attr_val);
                    }
                }
                Ok(Event::End(ref e)) => {
                    println!("End tag: {:?}", str::from_utf8(e.name().as_ref()).unwrap());
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }
    }
    fn is_num(value: &quick_xml::events::attributes::Attribute<'_>) -> bool {
        if let Ok(value) = value.unescape_value() {
            return value.trim().parse::<i32>().is_ok();
        }
        false
    }

    fn is_num_sequence(value: &quick_xml::events::attributes::Attribute<'_>) -> bool {
        if let Ok(value) = value.unescape_value() {
            return value.split(',').all(|x| x.trim().parse::<i32>().is_ok());
        }
        false
    }
}
