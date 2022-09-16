use std::io::Write;

fn prompt(prompt_text: &str) -> f32 {
    let mut line: String = String::new();
    print!("{}", prompt_text);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string().parse::<f32>().unwrap();
}

fn gain_from_stock(price: f32, target: f32) -> f32 {
    let profit: f32 = target - price;
    println!("The profit from holding a share is: {profit}");

    return profit;
}

fn gain_from_options(target: f32, ask: f32, strike: f32) -> f32 {
    let option_breakeven: f32 = strike + ask;
    let max_option_profit: f32 = target - option_breakeven;
    let option_profit: f32 = max_option_profit - ask;

    println!("The option breakeven is: {option_breakeven}");
    println!("The max option profit is: {max_option_profit}");
    println!("The option profit is: {option_profit}");

    return (target - option_breakeven) - ask;
}

fn compare<'life>(price: f32, target: f32, ask: f32, strike: f32) -> &'life str {
    let stock_gain: f32 = gain_from_stock(price, target);
    let option_gain: f32 = gain_from_options(target, ask, strike);

    let option_percentage_gain: f32 = option_gain / ask;
    let share_percentage_gain: f32 = stock_gain / price;
    let leverage_factor: f32 = option_percentage_gain / share_percentage_gain;

    println!("Option percentage gain: {option_percentage_gain}");
    println!("Share percentage gain: {share_percentage_gain}");
    println!("Leverage from options: {leverage_factor}");

    if leverage_factor > 1.0 {
        return "Options are more profitable" 
    } 

    return "Shares are more profitable";
}

fn main() {
    let current_share_price: f32 = prompt("Current share price: ");
    let price_target: f32 = prompt("Share Price target: ");
    let option_ask: f32 = prompt("Option ask: ");
    let strike: f32 = prompt("Option strike: ");
    
    let comparison_result: &str = compare(current_share_price, price_target, option_ask, strike);

    println!("{comparison_result}");
}
