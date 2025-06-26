use crate::sim::company::company::Company;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompanySnapshot {
    pub name: String,
    pub slogan: String,

}

impl From<&Company> for CompanySnapshot {
    fn from(company: &Company) -> Self {
        Self{
            name: company.name.clone(),
            slogan: company.slogan.clone() ,
        }
    }
}

impl From<Company> for CompanySnapshot {
    fn from(company: Company) -> Self {
        Self{
            name: company.name.clone(),
            slogan: company.slogan.clone() ,
        }
    }
}

impl PartialEq<&Company> for CompanySnapshot {
    fn eq(&self, other: &&Company) -> bool {
        self.name == other.name && self.slogan == other.slogan
    }
}