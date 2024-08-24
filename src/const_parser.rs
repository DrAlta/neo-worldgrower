use nom::{self, branch::alt, bytes::complete::{tag, take_until}, character::complete::{alphanumeric1, char, multispace0, one_of}, combinator::{map_res, opt, recognize}, multi::many1, sequence::tuple, IResult};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Nullablity{
    NotNullable,
    Nullable
}
//public static final PropertyCountMapProperty<MagicSpell> STUDYING_SPELLS = new PropertyCountMapProperty<MagicSpell>("studyingSpells", ALL_PROPERTIES);
pub fn parse_property_count_map_property(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (tail, (
        //  a,b,c
            a,b,r#type,
        //  d, 
            d, ident,
        f)) = tuple((
        multispace0,//a
        tag("public static final PropertyCountMapProperty<"),//b
        alphanumeric1,//c
        tag("> "),//d
        parse_token,//e
        tag(" = new PropertyCountMapProperty<"),//f
    ))(input)?;
    let(tail, _) = tag(r#type)(tail)?;
    match tuple((
        tag(">(\""),//h
        alphanumeric1,//i
        tag("\", ALL_PROPERTIES)"),//j
    ))(tail) {
        
        Ok((tail, (h, name, j))) => Ok((tail, (r#type, ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_personality_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final PersonalityProperty "),//b
        parse_token,//c
        tag(" = new PersonalityProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
            _
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_conditions_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final ConditionsProperty "),//b
        parse_token,//c
        tag(" = new ConditionsProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
            _
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_world_object_container_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final WorldObjectContainerProperty "),//b
        parse_token,//c
        tag(" = new WorldObjectContainerProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
            _
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_world_object_property(input: &str) -> IResult<&str, (&str, &str, Nullablity)> {
    match tuple((
        multispace0,//a
        tag("public static final WorldObjectProperty "),//b
        parse_token,//c
        tag(" = new WorldObjectProperty(\""),//d
        alphanumeric1,//e
        tag("\", "),
        parse_nullable,
        tag(", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
            _, nullable,
            _
    ))) => Ok((tail, (ident, name, nullable))),
        Err(err) => Err(err),
    }

}


pub fn parse_knowledge_map_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final KnowledgeMapProperty "),//b
        parse_token,//c
        tag(" = new KnowledgeMapProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
        _,
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_background_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final BackgroundProperty "),//b
        parse_token,//c
        tag(" = new BackgroundProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
        _,
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_id_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final IdProperty "),//b
        parse_token,//c
        tag(" = new IdProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
        _,
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_id_map_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final IdMapProperty "),//b
        parse_token,//c
        tag(" = new IdMapProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
        _,
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_id_list_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final IdListProperty "),//b
        parse_token,//c
        tag(" = new IdListProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES)"),//f
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
        _,
    ))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}


pub fn parse_boolean_property(input: &str) -> IResult<&str, (&str, &str, Nullablity)> {
    match tuple((
        multispace0,//a
        tag("public static final BooleanProperty "),//b
        parse_token,//c
        tag(" = new BooleanProperty(\""),//d
        alphanumeric1,//e
        tag("\", "),//f
        parse_nullable,//g
        tag(", ALL_PROPERTIES);"),//H
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
            _, nullable,
        _,
    ))) => Ok((tail, (ident, name, nullable))),
        Err(err) => Err(err),
    }

}


pub fn parse_buildings_list_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final BuildingsListProperty "),//b
        parse_token,//c
        tag(" = new BuildingsListProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES);"),//e

    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
            _))) => Ok((tail, (ident, name))),
        Err(err) => Err(err),
    }

}



pub fn parse_skill_property(input: &str) -> IResult<&str, (&str, &str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final SkillProperty "),//b
        parse_token,//c
        tag(" = new SkillProperty(\""),//d
        alphanumeric1,//e
        tag("\", ALL_PROPERTIES, \""),//e
        take_until("\");"),//f
        tag("\");")//g
    ))(input) {
        
        Ok((tail, (
            _,_, ident, 
            _, name, 
            _, desc,
            _))) => Ok((tail, (ident, name, desc))),
            /*
            Ok((tail, (
                _,_, _, ident, _,))) => Ok((tail, (ident, Nullable::NotNullable))),
    */
        Err(err) => Err(err),
    }

}


pub fn parse_un_checked_property(input: &str) -> IResult<&str, (&str, &str)> {
    match tuple((
        multispace0,//a
        tag("public static final UnCheckedProperty<"),//b
        alphanumeric1,//c
        tag("> "),//d
        parse_token,//e
        tag(" = new UnCheckedProperty<>(\""),//f
        alphanumeric1,//g
        tag("\", ALL_PROPERTIES);"),//h
    ))(input) {
        
        Ok((tail, (
            _,_, r#type, 
            _, ident, 
            _, _, _))) => Ok((tail, (r#type, ident))),
            /*
            Ok((tail, (
                _,_, _, ident, _,))) => Ok((tail, (ident, Nullable::NotNullable))),
    */
        Err(err) => Err(err),
    }

}


pub fn parse_string_property(input: &str) -> IResult<&str, (&str, Nullablity)> {
    match tuple((
        multispace0,//a
        tag("public static final StringProperty"),//b
        multispace0,//c
        parse_token,//d
        tag(" = new StringProperty(\""),//e
        parse_token,//f
        tag("\", "),//g
        parse_nullable,//h
        tag(", ALL_PROPERTIES);"),//k
    ))(input) {
        
        Ok((tail, (
            _,_, _, ident, 
            _, _,
            _, nullable,_))) => Ok((tail, (ident, nullable))),
            /*
            Ok((tail, (
                _,_, _, ident, _,))) => Ok((tail, (ident, Nullable::NotNullable))),
    */
        Err(err) => Err(err),
    }

}

pub fn parse_int_property(input: &str) -> IResult<&str, (&str, Option<i64>, Option<i64>, Nullablity)> {
    match tuple((
        multispace0,//a
        tag("public static final IntProperty"),//b
        multispace0,//c
        parse_token,//d
        tag(" = new IntProperty(\""),//e
        parse_token,//f
        tag("\", "),//g
        parse_option_int,//h
        tag(", "),//i
        parse_option_int,//j
        tag(", "),//k
        parse_nullable,//l
        tag(", ALL_PROPERTIES);"),//m
    ))(input) {
        Ok((tail, (
            _,_, _, ident,
            _, _, _, min, 
            _ , max, _, nullable,_))) => Ok((tail, (ident, min, max, nullable))),
        Err(err) => Err(err),
    }

}
fn parse_nullable(input: &str) -> IResult<&str, Nullablity> {
    alt((
        map_res(tag("NOT_NULLABLE"), |_| Ok::<_, ()>(Nullablity::NotNullable)),
        map_res(tag("NULLABLE"), |_| Ok::<_, ()>(Nullablity::Nullable)),
    ))(input)
}

fn parse_option_int(input: &str) -> IResult<&str, Option<i64>> {
    alt((
        map_res(tag("null"), |_| Ok::<_,()>(None)),
        map_res(
            recognize(
                tuple((
                    opt(char('-')),
                    many1(
                        one_of("0123456789_"),
                    )
                ))
            ),
            |out: &str| match i64::from_str_radix(&str::replace(&out, "_", ""), 10){
                Ok(x) => Ok(Some(x)),
                Result::Err(err) => Err(err),
            }
        )
    ))(input)
}

fn parse_token(input: &str) -> IResult<&str, &str>{
    recognize(
        many1(
            alt((
                alphanumeric1,
                tag("_")
            ))
        )
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;  

    #[test]
    fn test_parse_property_count_map_property(){
        let x = r#"    public static final PropertyCountMapProperty<MagicSpell> STUDYING_SPELLS = new PropertyCountMapProperty<MagicSpell>("studyingSpells", ALL_PROPERTIES);
        "#;
        let (_, r) = parse_property_count_map_property(x).unwrap();
        assert_eq!(r, ("MagicSpell", "STUDYING_SPELLS", "studyingSpells"))
    }


    #[test]
    fn test_parse_personality_property(){
        let x = r#"public static final PersonalityProperty PERSONALITY = new PersonalityProperty("personality", ALL_PROPERTIES);"#;
        let (_, r) = parse_personality_property(x).unwrap();
        assert_eq!(r, ("PERSONALITY", "personality"))
    }


    #[test]
    fn test_parse_conditions_property(){
        let x = r#"public static final ConditionsProperty CONDITIONS = new ConditionsProperty("conditions", ALL_PROPERTIES);"#;
        let (_, r) = parse_conditions_property(x).unwrap();
        assert_eq!(r, ("CONDITIONS", "conditions"))
    }


    #[test]
    fn test_parse_world_object_container_property(){
        let x = r#"public static final WorldObjectContainerProperty INVENTORY = new WorldObjectContainerProperty("inventory", ALL_PROPERTIES);"#;
        let (_, r) = parse_world_object_container_property(x).unwrap();
        assert_eq!(r, ("INVENTORY", "inventory"))
    }


    #[test]
    fn test_parse_world_object_property(){
        let x = r#"    public static final WorldObjectProperty FACADE = new WorldObjectProperty("facade", NULLABLE, ALL_PROPERTIES);
        "#;
        let (_, r) = parse_world_object_property(x).unwrap();
        assert_eq!(r, ("FACADE", "facade", Nullablity::Nullable))
    }


    #[test]
    fn test_parse_knowledge_map_property(){
        let x = r#"    public static final KnowledgeMapProperty KNOWLEDGE_MAP = new KnowledgeMapProperty("knowledgeMap", ALL_PROPERTIES);"#;
        let (_, r) = parse_knowledge_map_property(x).unwrap();
        assert_eq!(r, ("KNOWLEDGE_MAP", "knowledgeMap"))
    }


    #[test]
    fn test_parse_background_property(){
        let x = r#"    public static final BackgroundProperty BACKGROUND = new BackgroundProperty("background", ALL_PROPERTIES);"#;
        let (_, r) = parse_background_property(x).unwrap();
        assert_eq!(r, ("BACKGROUND", "background"))
    }


    #[test]
    fn test_parse_id_property(){
        let x = r#"public static final IdProperty MATE_ID = new IdProperty("mate", ALL_PROPERTIES);"#;
        let (_, r) = parse_id_property(x).unwrap();
        assert_eq!(r, ("MATE_ID", "mate"))
    }


    #[test]
    fn test_parse_id_map_property(){
        let x = r#"public static final IdMapProperty RELATIONSHIPS = new IdMapProperty("relationships", ALL_PROPERTIES);"#;
        let (_, r) = parse_id_map_property(x).unwrap();
        assert_eq!(r, ("RELATIONSHIPS", "relationships"))
    }


    #[test]
    fn test_parse_id_list_property(){
        let x = r#"    public static final IdListProperty GROUP = new IdListProperty("group", ALL_PROPERTIES);"#;
        let (_, r) = parse_id_list_property(x).unwrap();
        assert_eq!(r, ("GROUP", "group"))
    }


    #[test]
    fn test_parse_boolean_property(){
        let x = r#"    public static final BooleanProperty TWO_HANDED_WEAPON = new BooleanProperty("twoHandedWeapon", NOT_NULLABLE, ALL_PROPERTIES);"#;
        let (_, r) = parse_boolean_property(x).unwrap();
        assert_eq!(r, ("TWO_HANDED_WEAPON", "twoHandedWeapon", Nullablity::NotNullable))
    }


    #[test]
    fn test_parse_buildings_list_property(){
        let x = r#"    public static final BuildingsListProperty BUILDINGS = new BuildingsListProperty("buildings", ALL_PROPERTIES);"#;
        let (_, r) = parse_buildings_list_property(x).unwrap();
        assert_eq!(r, ("BUILDINGS", "buildings"))
    }


    #[test]
    fn test_parse_skill_property(){
        let x = r#"    public static final SkillProperty BLUFF_SKILL = new SkillProperty("bluff", ALL_PROPERTIES, "Used to mislead people during a conversion");        "#;
        let (_, r) = parse_skill_property(x).unwrap();
        assert_eq!(r, ("BLUFF_SKILL", "bluff", "Used to mislead people during a conversion"))
    }


    #[test]
    fn test_parse_un_checked_property(){
        let x = r#"public static final UnCheckedProperty<MetaInformation> META_INFORMATION = new UnCheckedProperty<>("metaInformation", ALL_PROPERTIES);"#;
        let (_, r) = parse_un_checked_property(x).unwrap();
        assert_eq!(r, ("MetaInformation", "META_INFORMATION"))
    }

    #[test]
    fn test_parse_string_property(){
        let x = r#"    public static final StringProperty NAME = new StringProperty("NAME", NOT_NULLABLE, ALL_PROPERTIES);"#;
        let (_, r) = parse_string_property(x).unwrap();
        assert_eq!(r, ("NAME", Nullablity::NotNullable))
    }

    #[test]
    fn test_parse_int_property(){
        let x = r#"public static final IntProperty DISTANCE = new IntProperty("distance", 0, null, NOT_NULLABLE, ALL_PROPERTIES);
        "#;
        let (_, r) = parse_int_property(x).unwrap();
        assert_eq!(r, ("DISTANCE", Some(0), None, Nullablity::NotNullable))
    }
}
