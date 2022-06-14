mod customer;

use std::io;
use crate::customer::{Customer, StoredAccounts};


fn main() {
    println!("Hello, world!");
    let mut new_store = customer::StoredAccounts::default();

    /*pub let acc_to = |mut new_store: StoredAccounts| -> &mut Customer{
        println!("Please enter account id to whom you want to transfer the money");
        print!("account id for transfer: ");
        let mut account_id_to = String::new();
        io::stdin().read_line(&mut account_id_to).expect("cannot read account input");
        let account_id_to: u64 = account_id_to.trim().parse().expect("wrong account number input");
        new_store.get_account(account_id_to)
    };*/

    /*pub fn acc_gotten(mut new_store: StoredAccounts) -> &mut Customer {
        println!("Please enter account id to whom you want to transfer the money");
        print!("account id for transfer: ");
        let mut account_id_to = String::new();
        io::stdin().read_line(&mut account_id_to).expect("cannot read account input");
        let account_id_to: u64 = account_id_to.trim().parse().expect("wrong account number input");
        new_store.get_account(account_id_to)

    }*/


    //bank_process(new_store);

    println!("Hello, how may i help you");
    println!("1. Create account");
    println!("2. My account");
    println!("3. Delete account");
    println!("4. Exit");
    println!("please enter value of choice:");

    let mut response1 = String::new();
    io::stdin().read_line(&mut response1).expect("cannot read input");
    let response1: u8 = response1.trim().parse().expect("wrong input");


    //let mut new_store = customer::StoredAccounts::default();
    //let acc = new_store.get_account(account_id_to);




    match response1 {
        1 => {
            println!("welcome to the bank");
            reg_new_client(new_store);
        },
        2 => {
            println!("welcome dear customer");
            bank_account(new_store);
        },
        3 => {
            account_delete(new_store);
        },
        4 => {
            println!("Thank you");
        },
        _=> {
            println!("please enter either 1 or 2");
        }
    }




    pub fn reg_new_client(mut stored_accounts: StoredAccounts) {
        println!("Please fill in the following details");
        println!("your name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("cannot read account input");
        println!();
        println!("national id: ");
        let mut national_id = String::new();
        io::stdin().read_line(&mut national_id).expect("cannot read account input");
        let national_id: u64 = national_id.trim().parse().expect("wrong national id number input");
        println!();
        println!("account id: ");
        let mut account_id = String::new();
        io::stdin().read_line(&mut account_id).expect("cannot read account input");
        let account_id: u64 = account_id.trim().parse().expect("wrong account id number input");
        println!();
        println!("your email: ");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("cannot read account input");
        println!();
        println!("amount: ");
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("cannot read account input");
        let amount: f64 = amount.trim().parse().expect("wrong amount input");


        let mut new_customer = customer::Customer::default();
        new_customer.register_account(name,national_id,account_id,email,amount);


        stored_accounts.save_account(new_customer);




    }

    fn bank_account(mut stored_accounts: StoredAccounts){
        println!("Please enter account id type the numbers 1234567 as default account is)");
        print!("account id: ");
        let mut account_id = String::new();
        io::stdin().read_line(&mut account_id).expect("cannot read account input");
        let account_id: u64 = account_id.trim().parse().expect("wrong account number input");

        let mut acc = stored_accounts.get_account(account_id);
        //let mut acc = acc_gotten(stored_accounts);

        //acc_to(stored_accounts);

        bank_menu(acc);


    }

    pub fn bank_menu(acc: &mut Customer){
        println!("welcome ");
        println!("How may we help you");
        println!("1. Update account details");
        println!("2. Deposit money");
        println!("3. Withdraw money");
        println!("4. Transfer money");
        println!("5. Account details");
        println!("6. Exit");
        println!("please enter value of choice:");

        let mut choice1 = String::new();
        io::stdin().read_line(&mut choice1).expect("cannot read input");
        let choice1: u8 = choice1.trim().parse().expect("wrong input");

        match choice1 {
            1 => {
                bank_acc_update(acc);
            },
            2 => {
                bank_deposit(acc);
            },
            3 => {
                bank_withdraw(acc);
            },
            4 => {
                //transfer_money(acc);
                println!("currently work in progress");
            },
            5 => {
                account_details(acc);
            },
            5 => {
                println!("Thank you!");
            },
            _=> {
                println!("please enter either 1, 2, 3 or 4");
            }
        }
    }

    pub fn bank_acc_update(acc: &mut Customer){
        println!("Which one do you want to update");
        println!("1. Name");
        println!("2. Email");
        println!("please enter value of choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("cannot read input");
        let choice: u8 = choice.trim().parse().expect("wrong input");

        match choice {
            1 => {
                edit_name(acc);
            },
            2 => {
                edit_email(acc);
            },
            _=>{
                println!("please enter 1, 2 or 3");
            }
        }

    }

    pub fn edit_name(acc: &mut Customer){
        println!("Type your name");
        print!("Name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Cannot read input");
        acc.change_name(name);
        another_option(acc);
    }

    pub fn edit_email(acc: &mut Customer){
        println!("Type your email");
        print!("Email: ");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Cannot read input");
        acc.change_email(email);
        another_option(acc);
    }

    pub fn bank_deposit(acc: &mut Customer){
        println!("How much do you want to deposit");
        print!("Amount: ");
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Cannot read input");
        let amount: f64 = amount.trim().parse().expect("wrong input");

        acc.deposit(amount);
        another_option(acc);
    }

    pub fn bank_withdraw(acc: &mut Customer){
        println!("How much do you want to withdraw");
        print!("Amount: ");
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Cannot read input");
        let amount: f64 = amount.trim().parse().expect("wrong input");

        acc.withdraw(amount);
        another_option(acc);

    }



    pub fn transfer_money(acc: &mut Customer, mut store: StoredAccounts){

         /*println!("Please enter account id to whom you want to transfer the money");
         print!("account id for transfer: ");
         let mut account_id_to = String::new();
         io::stdin().read_line(&mut account_id_to).expect("cannot read account input");
         let account_id_to: u64 = account_id_to.trim().parse().expect("wrong account number input");


        let acc_to = acc_gotten(store);

        //let acc_to = || { new_store.get_account(account_id_to) };

         println!("How much do you want to transfer");
         print!("Amount: ");
         let mut amount = String::new();
         io::stdin().read_line(&mut amount).expect("Cannot read input");
         let amount: f64 = amount.trim().parse().expect("wrong input");


         acc.withdraw(amount);
         acc_to.deposit(amount);*/
        println!("work in progress");

        another_option(acc);

        //work on it

    }

    pub fn account_details(acc: &mut Customer){
        println!("{:?}", &acc);
        another_option(acc);
    }

    pub fn account_delete(mut stored_accounts: StoredAccounts){
        println!("Please enter account id");
        print!("account id: ");
        let mut account_id = String::new();
        io::stdin().read_line(&mut account_id).expect("cannot read account input");
        let account_id: u64 = account_id.trim().parse().expect("wrong account number input");

        //let acc = stored_accounts.get_account(account_id);
        stored_accounts.delete_account(account_id);



    }

    pub fn another_option(acc: &mut Customer){


        let mut response = String::new();
        println!("Would you want to check out another option?");
        println!("1.yes");
        println!("2.no");
        println!("please enter value of choice:");
        io::stdin().read_line(&mut response).expect("Cannot read value");
        let response: u32 = response.trim().parse().expect("wrong input");

        if response == 1{
            bank_menu(acc);
        } else {
            print!("Thank you");
        }


    }



}




/*fn bank_process(new_store: StoredAccounts){
    println!("Hello, how may i help you");
    println!("1. Create account");
    println!("2. My account");
    println!("3. Delete account");
    println!("4. Exit");
    println!("please enter value of choice:");

    let mut response1 = String::new();
    io::stdin().read_line(&mut response1).expect("cannot read input");
    let response1: u8 = response1.trim().parse().expect("wrong input");


    //let mut new_store = customer::StoredAccounts::default();



    match response1 {
        1 => {
            println!("welcome to the bank");
            reg_new_client(new_store);
        },
        2 => {
            println!("welcome dear customer");
            bank_account(new_store);
        },
        3 => {
            account_delete(new_store);
        },
        4 => {
            println!("welcome dear customer");
            bank_account(new_store);
        },
        _=> {
            println!("please enter either 1 or 2");
        }
    }
}*/

