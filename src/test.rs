use super::*;

fn assert_close(actual: f64, expected: f64)
{
    assert!((actual - expected).abs() < 1e-9, "expected {}, got {}", expected, actual);
}

#[test]
fn average_price_is_weighted_by_shares()
{
    let transactions = vec![
        SharesPrice { shares: 415, price: 23.65 },
        SharesPrice { shares: 138, price: 16.50 },
    ];

    let expected = (415.0 * 23.65 + 138.0 * 16.50) / 553.0;
    assert_close(calculate_average_price(transactions), expected);
}

#[test]
fn percentage_of_a_value()
{
    assert_close(calculate_percentage_of(25.0, 200.0), 12.5);
}

#[test]
fn currency_conversion_in_both_directions()
{
    assert_close(convert_from_orig(50.0, 1.2), 60.0);
    assert_close(convert_to_orig(60.0, 1.2), 50.0);
}

#[test]
fn recommended_shares_account_for_tax_and_commission_and_truncate()
{
    assert_eq!(calculate_shares_recommended(1000.0, 10.0, 1.0, 100.0), 9);
}

#[test]
fn leveraged_contracts_round_up_in_groups_of_three()
{
    assert_eq!(calculate_leveraged_contracts(1), 1);
    assert_eq!(calculate_leveraged_contracts(3), 3);
    assert_eq!(calculate_leveraged_contracts(4), 5);
}

#[test]
fn stoploss_for_long_positions()
{
    assert_close(calculate_stoploss(100.0, 10, 0.0, 5.0, 100.0, 10, 0.0, 5.0, 2.0, 1000.0, TradeType::Long), 99.0);
}

#[test]
fn stoploss_for_short_positions()
{
    assert_close(calculate_stoploss(100.0, 10, 0.0, 5.0, 100.0, 10, 0.0, 5.0, 2.0, 1000.0, TradeType::Short), 101.0);
    let stoploss = calculate_stoploss(100.0, 10, 1.0, 5.0, 100.0, 10, 2.0, 7.0, 8.24, 1000.0, TradeType::Short);
    assert_close(stoploss, 104.0);
    assert_close(calculate_risk_percentage(100.0, 10, 1.0, 5.0, 100.0, 10, 2.0, 7.0, stoploss, 1000.0, TradeType::Short), 8.24);
}

#[test]
fn initial_stoploss_for_short_positions()
{
    let stoploss = calculate_stoploss_initial(0.0, 0, 0.0, 0.0, 100.0, 10, 5.0, 1.0, 14.4, 1000.0, TradeType::Short);
    assert_close(stoploss, 104.0);
    assert_close(calculate_risk_initial(0.0, 0, 0.0, 0.0, 100.0, 10, 5.0, 1.0, stoploss, TradeType::Short), 144.0);
}

#[test]
fn risk_percentage_accounts_for_tax_and_commission_on_both_trade_types()
{
    assert_close(calculate_risk_percentage(100.0, 10, 1.0, 5.0, 100.0, 10, 2.0, 7.0, 96.0, 1000.0, TradeType::Long), 8.12);
    assert_close(calculate_risk_percentage(100.0, 10, 1.0, 5.0, 100.0, 10, 2.0, 7.0, 104.0, 1000.0, TradeType::Short), 8.24);
}

#[test]
fn risk_percentage_accounts_for_tax_and_commission_on_both_trade_types_for_input()
{
    assert_close(calculate_risk_percentage_initial(100.0, 10, 1.0, 5.0, 0.0, 0, 0.0, 0.0, 96.0, 1000.0, TradeType::Long), 6.960000000000002);
    assert_close(calculate_risk_percentage_initial(0.0, 0, 0.0, 0.0, 100.0, 10, 5.0, 1.0, 104.0, 1000.0, TradeType::Short), 14.4);
}

#[test]
fn input_risk_is_a_percentage_of_the_pool()
{
    assert_close(calculate_risk_input(1000.0, 5.0), 50.0);
}

#[test]
fn initial_risk_for_long_and_short_positions()
{
    assert_close(calculate_risk_initial(100.0, 10, 0.0, 5.0, 0.0, 0, 0.0, 0.0, 96.0, TradeType::Long), 50.0);
    assert_close(calculate_risk_initial(0.0, 0, 0.0, 0.0, 100.0, 10, 0.0, 5.0, 104.0, TradeType::Short), 50.0);
}

#[test]
fn amount_is_price_times_shares()
{
    assert_close(calculate_amount(12.5, 8), 100.0);
}

#[test]
fn amount_with_tax_and_commission_for_buy_and_sell()
{
    assert_close(calculate_amount_with_tax_and_commission(20.0, 5, 2.0, 3.0, TransactionType::Buy), 105.0);
    assert_close(calculate_amount_with_tax_and_commission(20.0, 5, 2.0, 3.0, TransactionType::Sell), 95.0);
}

#[test]
fn transaction_cost_includes_tax_and_commission()
{
    assert_close(cost_transaction(20.0, 5, 2.0, 3.0), 5.0);
}

#[test]
fn tax_cost_for_buy_and_sell_amounts()
{
    assert_close(cost_tax(105.0, 3.0, 5, 20.0, TransactionType::Buy), 2.0);
    assert_close(cost_tax(95.0, 3.0, 5, 20.0, TransactionType::Sell), 2.0);
}

#[test]
fn price_from_buy_and_sell_amounts()
{
    assert_close(calculate_price(105.0, 5, 2.0, 3.0, TransactionType::Buy), 20.0);
    assert_close(calculate_price(95.0, 5, 2.0, 3.0, TransactionType::Sell), 20.0);
}

#[test]
fn actual_risk_uses_initial_risk_when_loss_is_within_it_or_trade_is_profitable()
{
    assert_close(calculate_risk(100.0, 10, 1.0, 5.0, 90.0, 10, 2.0, 7.0, 150.0, -100.0), 150.0);
    assert_close(calculate_risk(100.0, 10, 1.0, 5.0, 110.0, 10, 2.0, 7.0, 150.0, 100.0), 150.0);
}

#[test]
fn actual_risk_uses_trade_amounts_when_loss_exceeds_initial_risk()
{
    assert_close(calculate_risk(100.0, 10, 1.0, 5.0, 90.0, 10, 2.0, 7.0, 100.0, -150.0), 140.0);
}

#[test]
fn r_multiple_is_profit_or_loss_divided_by_initial_risk()
{
    assert_close(calculate_r_multiple(-75.0, 50.0), -1.5);
}

#[test]
fn total_cost_includes_both_sides_of_the_trade()
{
    assert_close(calculate_cost_total(1000.0, 1.0, 5.0, 1200.0, 2.0, 7.0), 46.0);
}

#[test]
fn profit_loss_uses_buy_and_sell_amounts()
{
    assert_close(calculate_profit_loss(100.0, 10, 120.0, 10), 200.0);
}

#[test]
fn total_profit_loss_deducts_commissions()
{
    assert_close(calculate_profit_loss_total(100.0, 10, 0.0, 5.0, 120.0, 10, 0.0, 7.0), 188.0);
}

#[test]
fn calculate_profit_loss_works()
{
    assert_close(calculate_profit_loss(100.0, 10, 120.0, 10), 200.0);
    assert_close(calculate_profit_loss(120.0, 10, 100.0, 10), -200.0);
}

#[test]
fn calculate_profit_loss_total_works()
{
    assert_close(calculate_profit_loss_total(100.0, 10, 5.0, 10.0, 120.0, 10, 5.0, 10.0), 70.0);
    assert_close(calculate_profit_loss_total(120.0, 10, 5.0, 10.0, 100.0, 10, 5.0, 10.0), -330.0);
}

#[test]
fn other_cost_is_the_remaining_difference_or_zero()
{
    assert_close(calculate_cost_other(200.0, 180.0, 15.0), 5.0);
    assert_close(calculate_cost_other(200.0, 180.0, 20.0), 0.0);
}

#[test]
fn calculate_close_at_breakeven_works_long()
{
    assert_close(calculate_close_at_breakeven(100.0, 10, 5.0, 10.0, 0.0, 0, 0.0, 0.0, TradeType::Long), 112.63157894736842);
}

#[test]
fn calculate_close_at_breakeven_works_short()
{
    let price = calculate_close_at_breakeven(0.0, 0, 0.0, 0.0, 100.0, 10, 5.0, 10.0, TradeType::Short);
    assert_close(price, 88.57142857142857);
    assert_close(calculate_profit_loss_total(price, 10, 5.0, 10.0, 100.0, 10, 5.0, 10.0), 0.0);
}

#[test]
fn calculate_close_at_preferred_works_long()
{
    assert_close(calculate_close_at_preferred(100.0, 10, 5.0, 10.0, 0.0, 0, 0.0, 0.0, 1000.0, TradeType::Long), 217.89473684210526);
}

#[test]
fn calculate_close_at_preferred_works_short()
{
    let price = calculate_close_at_preferred(0.0, 0, 0.0, 0.0, 100.0, 10, 5.0, 10.0, 100.0, TradeType::Short);
    assert_close(price, 79.04761904761905);
    assert_close(calculate_profit_loss_total(price, 10, 5.0, 10.0, 100.0, 10, 5.0, 10.0), 100.0);
}

#[test]
fn contract_factor_scales_amounts_costs_and_profit()
{
    let factor = 5.0; // MES: USD 5 per index point per contract
    assert_eq!(calculate_shares_recommended_with_factor(100000.0, 0.0, 0.0, 5000.0, factor), 4);
    assert_close(calculate_amount_with_factor(5000.0, 2, factor), 50000.0);
    assert_close(calculate_amount_with_tax_and_commission_with_factor(5000.0, 2, 1.0, 2.0, TransactionType::Buy, factor), 50502.0);
    assert_close(calculate_amount_with_tax_and_commission_with_factor(5010.0, 2, 1.0, 2.0, TransactionType::Sell, factor), 49597.0);
    assert_close(cost_transaction_with_factor(5000.0, 2, 1.0, 2.0, factor), 502.0);
    assert_close(cost_tax_with_factor(50502.0, 2.0, 2, 5000.0, TransactionType::Buy, factor), 500.0);
    assert_close(cost_tax_with_factor(49597.0, 2.0, 2, 5010.0, TransactionType::Sell, factor), 501.0);
    assert_close(calculate_price_with_factor(50502.0, 2, 1.0, 2.0, TransactionType::Buy, factor), 5000.0);
    assert_close(calculate_price_with_factor(49597.0, 2, 1.0, 2.0, TransactionType::Sell, factor), 5010.0);
    assert_close(calculate_profit_loss_with_factor(5000.0, 2, 5010.0, 2, factor), 100.0);
    assert_close(calculate_profit_loss_total_with_factor(5000.0, 2, 0.0, 2.0, 5010.0, 2, 0.0, 2.0, factor), 96.0);
}

#[test]
fn contract_factor_scales_risk_and_returns_index_prices()
{
    let factor = 5.0;
    let long_stop = calculate_stoploss_with_factor(5000.0, 2, 0.0, 2.0, 5000.0, 2, 0.0, 2.0, 1.0, 10000.0, TradeType::Long, factor);
    let short_stop = calculate_stoploss_with_factor(5000.0, 2, 0.0, 2.0, 5000.0, 2, 0.0, 2.0, 1.0, 10000.0, TradeType::Short, factor);
    assert_close(long_stop, 4990.4);
    assert_close(short_stop, 5009.6);
    assert_close(calculate_stoploss_initial_with_factor(5000.0, 2, 0.0, 2.0, 0.0, 0, 0.0, 0.0, 1.0, 10000.0, TradeType::Long, factor), long_stop);
    assert_close(calculate_stoploss_initial_with_factor(0.0, 0, 0.0, 0.0, 5000.0, 2, 0.0, 2.0, 1.0, 10000.0, TradeType::Short, factor), short_stop);
    assert_close(calculate_risk_initial_with_factor(5000.0, 2, 0.0, 2.0, 0.0, 0, 0.0, 0.0, long_stop, TradeType::Long, factor), 100.0);
    assert_close(calculate_risk_percentage_initial_with_factor(5000.0, 2, 0.0, 2.0, 0.0, 0, 0.0, 0.0, long_stop, 10000.0, TradeType::Long, factor), 1.0);
    assert_close(calculate_risk_percentage_with_factor(5000.0, 2, 0.0, 2.0, 5000.0, 2, 0.0, 2.0, short_stop, 10000.0, TradeType::Short, factor), 1.0);
    assert_close(calculate_risk_with_factor(5000.0, 2, 0.0, 2.0, 4990.0, 2, 0.0, 2.0, 50.0, -104.0, factor), 104.0);
    assert_close(calculate_close_at_breakeven_with_factor(5000.0, 2, 0.0, 2.0, 0.0, 0, 0.0, 0.0, TradeType::Long, factor), 5000.4);
    assert_close(calculate_close_at_preferred_with_factor(5000.0, 2, 0.0, 2.0, 0.0, 0, 0.0, 0.0, 100.0, TradeType::Long, factor), 5010.4);
    assert_close(calculate_close_at_breakeven_with_factor(0.0, 0, 0.0, 0.0, 5000.0, 2, 0.0, 2.0, TradeType::Short, factor), 4999.6);
    assert_close(calculate_close_at_preferred_with_factor(0.0, 0, 0.0, 0.0, 5000.0, 2, 0.0, 2.0, 100.0, TradeType::Short, factor), 4989.6);
}
