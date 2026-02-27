fn main(){
    let mut balance:u32=100;

    add_money(&mut balance, 300);
    take_money(&mut balance, 30);
    view_balance(&balance);

}

fn add_money(balance:&mut u32,amt:u32){
    // * is used to get the value from the reference var
    // then we return the value.
    *balance+=amt
}

fn take_money(balance:&mut u32,amt:u32){
    // * is used to get the value from the reference var
    // then we return the value.
    if amt > *balance{
        println!("Sorry not enough balance, withdrawl failed!");
        return
    }
    *balance-=amt;
}


fn view_balance(balance:&u32){
    // we dont use mut ptr beocz it is read
    println!("{balance}");
}