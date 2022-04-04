#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
pub trait Husband {
    type Partner: Wife<Partner = Self>;
    fn get_marry(&mut self, partner: &Self::Partner);
    fn who_is_my_partner(&self) -> Self::Partner;
}

pub trait Wife {
    type Partner: Husband<Partner = Self>;
    fn get_marry(&mut self, partner: &Self::Partner);
    fn who_is_my_partner(&self) -> Self::Partner;
}
#[derive(Debug)]
pub struct ChineseMan {
    pub name: String,
    pub partner_name: Option<String>,
}
#[derive(Debug)]
pub struct ChineseWoman {
    pub name: String,
    pub partner_name: Option<String>,
}

impl Husband for ChineseMan {
    type Partner = ChineseWoman;
    fn get_marry(&mut self, partner: &Self::Partner) {
        self.partner_name = Some(partner.name.clone());
    }
    fn who_is_my_partner(&self) -> Self::Partner {
        Self::Partner {
            name: self.partner_name.clone().unwrap(),
            partner_name: Some(self.name.clone()),
        }
    }
}
impl Wife for ChineseWoman {
    type Partner = ChineseMan;
    fn get_marry(&mut self, partner: &Self::Partner) {
        self.partner_name = Some(partner.name.clone());
    }
    fn who_is_my_partner(&self) -> Self::Partner {
        Self::Partner {
            name: self.partner_name.clone().unwrap(),
            partner_name: Some(self.name.clone()),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut cm = ChineseMan {
            name: "zhangsan".to_string(),
            partner_name: None,
        };
        let mut cwm = ChineseWoman {
            name: "lisi".to_string(),
            partner_name: None,
        };

        church(&mut cm, &mut cwm);
        println!("after married");
        println!("{:?}", cm.who_is_my_partner());
        println!("{:?}", cwm.who_is_my_partner());
    }

    pub fn church(cm: &mut ChineseMan, cwm: &mut ChineseWoman) {
        cm.get_marry(cwm);
        cwm.get_marry(cm);
    }
}
