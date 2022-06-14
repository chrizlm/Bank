#[derive(Debug)]
pub struct Customer{
    pub(crate) name: String,
    pub(crate) national_id: u64,
    pub(crate) acc_number: u64,
    pub(crate) email: String,
    pub(crate) amount: f64
}

const DEFAULT_NAME: &str = "new_customer";
const DEFAULT_NATIONAL_ID: u64 = 1234567;
const DEFAULT_ACC_NO: u64 = 1234567;
const DEFAULT_EMAIL: &str = "new@gmail.com";
const DEFAULT_AMOUNT: f64 = 0.0;

impl Default for Customer {
    fn default() -> Self{
        Self{name: DEFAULT_NAME.to_string(),
        national_id: DEFAULT_NATIONAL_ID,
        acc_number: DEFAULT_ACC_NO,
        email: DEFAULT_EMAIL.to_string(),
        amount: DEFAULT_AMOUNT}
    }
}

impl Customer{

    pub fn register_account(&mut self, name: String,
                            national_id: u64,
                            acc_number: u64,
                            email: String,
                            amount: f64){
        self.name = name;
        self.national_id = national_id;
        self.acc_number = acc_number;
        self.email = email;
        self.amount = amount;


    }





    pub fn deposit(&mut self, amount: f64){
        self.amount += amount;
    }


    pub fn withdraw(&mut self, amount: f64){
        if self.amount > amount{
            self.amount -= amount;
        }
    }

    pub fn change_name(&mut self, name: String){
        self.name = name;
    }

    pub fn change_email(&mut self, email: String){
        self.email = email;
    }



}



pub struct StoredAccounts{
    pub(crate) storage: Vec<Customer>
}

//const DEFAULT_CUSTOMER: Customer = Customer::default();
const DEFAULT_STORAGE: Vec<Customer> = vec![];

impl Default for StoredAccounts {
    fn default() -> Self{
        let new_customer = Customer::default();
        let mut new_vec = Vec::new();
        new_vec.push(new_customer);
        Self{storage: new_vec,
            }
    }
}

impl StoredAccounts{
    pub fn save_account(&mut self, account: Customer){
        println!("{:?}", &account);

        self.storage.push(account);

    }

/*     pub fn index_p(&self, acc_num: u64) -> usize{
        let index = self.storage.iter().position(|x| x.acc_number == acc_num).unwrap();
        index
    } */



    pub fn get_account(&mut self, acc_num: u64) -> &mut Customer {
        let index = self.storage.iter().position(|x| x.acc_number == acc_num).unwrap();
        let mut account = &mut self.storage[index];

        account
    }


    pub fn get_account2(&mut self, acc_num: u64) -> &Customer {
        let index = self.storage.iter().position(|x| x.acc_number == acc_num).unwrap();
        let mut account = &self.storage[index];

        account
    }

    pub fn delete_account(&mut self, acc_num: u64){
        let index = self.storage.iter().position(|x| x.acc_number == acc_num).unwrap();
        self.storage.remove(index);
    }


}


