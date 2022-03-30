
pub trait Armor {
    type RelatedWeapon: Weapon<RelatedArmor = Self>;
    type RelatedShield: Shield<RelatedArmor = Self>;  
    fn get_all_equitment(&self) -> (Self::RelatedWeapon, Self::RelatedShield);
}
pub trait Weapon {
    type RelatedArmor: Armor<RelatedWeapon = Self>;
    type RelatedShield: Shield<RelatedWeapon = Self>;

}
pub trait Shield {
    type RelatedArmor: Armor<RelatedShield = Self>;
    type RelatedWeapon: Weapon<RelatedShield = Self>;

}
#[derive(Default,Debug)]
pub struct GoldArmor;
#[derive(Default,Debug)]
pub struct GoldWeapon;
#[derive(Default,Debug)]
pub struct GoldShield;

impl Armor for GoldArmor {
    type RelatedWeapon = GoldWeapon;
    type RelatedShield = GoldShield;
    fn get_all_equitment(&self) -> (Self::RelatedWeapon, Self::RelatedShield) {
        (Self::RelatedWeapon::default(), Self::RelatedShield::default())
    }

}

impl Weapon for GoldWeapon {
    type RelatedArmor = GoldArmor;
    type RelatedShield = GoldShield;
}

impl Shield for GoldShield {
    type RelatedArmor = GoldArmor;
    type RelatedWeapon = GoldWeapon;
}
    

fn main() {
    let a = GoldArmor::default();
    let (weapon, shield) = a.get_all_equitment();
    println!("{:?}", weapon);
    println!("{:?}", shield);
   
}
