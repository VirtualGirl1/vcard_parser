use std::fmt::{Display, Formatter};

use uuid::Uuid;

use crate::vcard::parameter::Parameter;
use crate::vcard::property::types::{
    PropertyType, PROPERTY_TYPE_ADR, PROPERTY_TYPE_ANNIVERSARY, PROPERTY_TYPE_BDAY, PROPERTY_TYPE_BIRTHPLACE, PROPERTY_TYPE_CALADRURI, PROPERTY_TYPE_CALURI, PROPERTY_TYPE_CATEGORIES, PROPERTY_TYPE_CLIENTPIDMAP, PROPERTY_TYPE_CONTACTURI, PROPERTY_TYPE_DEATHDATE, PROPERTY_TYPE_DEATHPLACE, PROPERTY_TYPE_EMAIL, PROPERTY_TYPE_EXPERTISE, PROPERTY_TYPE_FBURL, PROPERTY_TYPE_FN, PROPERTY_TYPE_GENDER, PROPERTY_TYPE_GEO, PROPERTY_TYPE_HOBBY, PROPERTY_TYPE_IMPP, PROPERTY_TYPE_INTEREST, PROPERTY_TYPE_KEY,
    PROPERTY_TYPE_KIND, PROPERTY_TYPE_LANG, PROPERTY_TYPE_LOGO, PROPERTY_TYPE_MEMBER, PROPERTY_TYPE_N, PROPERTY_TYPE_NICKNAME, PROPERTY_TYPE_NOTE, PROPERTY_TYPE_ORG, PROPERTY_TYPE_ORGDIRECTORY, PROPERTY_TYPE_PHOTO, PROPERTY_TYPE_PRODID, PROPERTY_TYPE_RELATED, PROPERTY_TYPE_REV, PROPERTY_TYPE_ROLE, PROPERTY_TYPE_SOUND, PROPERTY_TYPE_SOURCE, PROPERTY_TYPE_TEL, PROPERTY_TYPE_TITLE, PROPERTY_TYPE_TZ, PROPERTY_TYPE_UID, PROPERTY_TYPE_URL, PROPERTY_TYPE_VERSION, PROPERTY_TYPE_XML,
};
use crate::vcard::values::Value;
use crate::VcardError;

/// Stores the property type as an enum variant.
pub mod types;

/// Stores property data including type, parameter and value. Includes an autogenerated uuid for convenient lookup.
/// Normally you won't create properties manually, rather you would use the Vcard implementations for [adding](super::Vcard::add_property)
/// and [updating](super::Vcard::update_property) the property instead, as vcard properties are immutable.
///
/// # Examples
/// ```
/// use vcard_parser::vcard::property::Property;
/// use vcard_parser::vcard::property::types::PropertyType;
///
/// let property = Property::from(PropertyType::Version);
/// assert_eq!(property.to_string(), "VERSION:4.0");
/// ```
#[derive(Clone)]
pub struct Property {
    uuid: Uuid,
    property_type: PropertyType,
    property_value: Value,
    property_parameters: Vec<Parameter>,
}

impl Property {
    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    pub fn get_type(&self) -> &PropertyType {
        &self.property_type
    }
    pub fn get_value(&self) -> &Value {
        &self.property_value
    }
    pub fn get_parameters(&self) -> &Vec<Parameter> {
        &self.property_parameters
    }
    pub fn parameters_to_string(&self) -> String {
        let parameters: Vec<String> = self.property_parameters.iter().map(|p| p.to_string()).collect();
        parameters.join(";")
    }
}

impl From<PropertyType> for Property {
    fn from(property_type: PropertyType) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            property_value: Value::from(&property_type),
            property_parameters: Vec::new(),
            property_type,
        }
    }
}

impl TryFrom<(&str, Option<Uuid>)> for Property {
    type Error = VcardError;
    fn try_from((str, uuid): (&str, Option<Uuid>)) -> Result<Self, Self::Error> {
        let str = str.replace('\r', "").replace('\n', "");
        let (pt, pv, pp) = match str.split_once(':') {
            None => Err(VcardError::PropertyMalformedString(str.to_string())),
            Some((rest, property_values)) => match rest.split_once(';') {
                None => Ok((rest, property_values, None)),
                Some((property_type, property_parameters)) => Ok((property_type, property_values, Some(property_parameters))),
            },
        }?;

        let property_type = PropertyType::try_from(pt)?;
        let property_parameters = Parameter::build_parameters(&property_type, pp)?;
        let property_value = Value::try_from((&property_type, &property_parameters, pv))?;

        Ok(Self {
            uuid: uuid.unwrap_or_else(Uuid::new_v4),
            property_type,
            property_value,
            property_parameters,
        })
    }
}

impl TryFrom<(&PropertyType, &str, Option<Uuid>)> for Property {
    type Error = VcardError;
    fn try_from((property_type, str, uuid): (&PropertyType, &str, Option<Uuid>)) -> Result<Self, Self::Error> {
        let str = match property_type {
            PropertyType::Adr => format!("{}:{}", PROPERTY_TYPE_ADR, str),
            PropertyType::Anniversary => format!("{}:{}", PROPERTY_TYPE_ANNIVERSARY, str),
            PropertyType::BDay => format!("{}:{}", PROPERTY_TYPE_BDAY, str),
            PropertyType::BirthPlace => format!("{}:{}", PROPERTY_TYPE_BIRTHPLACE, str),
            PropertyType::CalAdrUri => format!("{}:{}", PROPERTY_TYPE_CALADRURI, str),
            PropertyType::CalUri => format!("{}:{}", PROPERTY_TYPE_CALURI, str),
            PropertyType::Categories => format!("{}:{}", PROPERTY_TYPE_CATEGORIES, str),
            PropertyType::ClientPidMap => format!("{}:{}", PROPERTY_TYPE_CLIENTPIDMAP, str),
            PropertyType::ContactUri => format!("{}:{}", PROPERTY_TYPE_CONTACTURI, str),
            PropertyType::DeathDate => format!("{}:{}", PROPERTY_TYPE_DEATHDATE, str),
            PropertyType::DeathPlace => format!("{}:{}", PROPERTY_TYPE_DEATHPLACE, str),
            PropertyType::Email => format!("{}:{}", PROPERTY_TYPE_EMAIL, str),
            PropertyType::Expertise => format!("{}:{}", PROPERTY_TYPE_EXPERTISE, str),
            PropertyType::FbUrl => format!("{}:{}", PROPERTY_TYPE_FBURL, str),
            PropertyType::Fn => format!("{}:{}", PROPERTY_TYPE_FN, str),
            PropertyType::Gender => format!("{}:{}", PROPERTY_TYPE_GENDER, str),
            PropertyType::Geo => format!("{}:{}", PROPERTY_TYPE_GEO, str),
            PropertyType::Hobby => format!("{}:{}", PROPERTY_TYPE_HOBBY, str),
            PropertyType::Impp => format!("{}:{}", PROPERTY_TYPE_IMPP, str),
            PropertyType::Interest => format!("{}:{}", PROPERTY_TYPE_INTEREST, str),
            PropertyType::Key => format!("{}:{}", PROPERTY_TYPE_KEY, str),
            PropertyType::Kind => format!("{}:{}", PROPERTY_TYPE_KIND, str),
            PropertyType::Lang => format!("{}:{}", PROPERTY_TYPE_LANG, str),
            PropertyType::Logo => format!("{}:{}", PROPERTY_TYPE_LOGO, str),
            PropertyType::Member => format!("{}:{}", PROPERTY_TYPE_MEMBER, str),
            PropertyType::NickName => format!("{}:{}", PROPERTY_TYPE_NICKNAME, str),
            PropertyType::Note => format!("{}:{}", PROPERTY_TYPE_NOTE, str),
            PropertyType::N => format!("{}:{}", PROPERTY_TYPE_N, str),
            PropertyType::OrgDirectory => format!("{}:{}", PROPERTY_TYPE_ORGDIRECTORY, str),
            PropertyType::Org => format!("{}:{}", PROPERTY_TYPE_ORG, str),
            PropertyType::Photo => format!("{}:{}", PROPERTY_TYPE_PHOTO, str),
            PropertyType::ProdId => format!("{}:{}", PROPERTY_TYPE_PRODID, str),
            PropertyType::Related => format!("{}:{}", PROPERTY_TYPE_RELATED, str),
            PropertyType::Rev => format!("{}:{}", PROPERTY_TYPE_REV, str),
            PropertyType::Role => format!("{}:{}", PROPERTY_TYPE_ROLE, str),
            PropertyType::Sound => format!("{}:{}", PROPERTY_TYPE_SOUND, str),
            PropertyType::Source => format!("{}:{}", PROPERTY_TYPE_SOURCE, str),
            PropertyType::Tel => format!("{}:{}", PROPERTY_TYPE_TEL, str),
            PropertyType::Title => format!("{}:{}", PROPERTY_TYPE_TITLE, str),
            PropertyType::Tz => format!("{}:{}", PROPERTY_TYPE_TZ, str),
            PropertyType::Uid => format!("{}:{}", PROPERTY_TYPE_UID, str),
            PropertyType::Url => format!("{}:{}", PROPERTY_TYPE_URL, str),
            PropertyType::Version => format!("{}:{}", PROPERTY_TYPE_VERSION, str),
            PropertyType::Xml => format!("{}:{}", PROPERTY_TYPE_XML, str),
        };
        Property::try_from((str.as_str(), uuid))
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.property_parameters.is_empty() {
            write!(f, "{};{}:{}", self.property_type, self.parameters_to_string(), self.property_value)
        } else {
            write!(f, "{}:{}", self.property_type, self.property_value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::property::types::PropertyType;
    use crate::vcard::property::Property;

    #[test]
    pub fn property_formatting() {
        let text = "ADR;TYPE=HOME;TYPE=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States";
        assert_eq!(Property::try_from((text, None)).unwrap().to_string(), text);

        assert!(matches!(Property::try_from((&PropertyType::Adr, ";;;;;;", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Anniversary, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::BDay, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::BirthPlace, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::CalAdrUri, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::CalUri, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Categories, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::ClientPidMap, "1;some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::ContactUri, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::DeathDate, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::DeathPlace, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Email, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Expertise, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::FbUrl, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Fn, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Gender, "M;M", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Geo, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Hobby, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Impp, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Interest, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Key, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Kind, "individual", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Lang, "en", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Logo, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Member, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::NickName, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Note, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::N, ";;;;", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::OrgDirectory, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Org, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Photo, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::ProdId, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Related, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Rev, "19961022T140000", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Role, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Sound, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Source, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Tel, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Title, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Tz, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Uid, "", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Url, "some:url", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Version, "4.0", None)), Ok(_)));
        assert!(matches!(Property::try_from((&PropertyType::Xml, "", None)), Ok(_)));
    }
}
