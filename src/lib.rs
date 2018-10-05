/* See LICENSE.txt file for license and copyright information. */

/**********************************************************************
 * calculate_average_price:
 * Calculate the average price, based on previous transactions.
 * It requires a SharesPrice struct list, preceded by the total number
 * of records.
 * Note:
 * -----
 * S1 = 415, P1 = 23.65, S2 = 138, P2 = 16.50
 * When you need to buy new stocks and you need to log this for
 * accounting purposes, you need to know what the average price was
 * that you paid. You only know the total number of shares you have,
 * called S3. The price is the average of all prices you paid on buy/
 * sell of all previous transactions.
 * S1 * P1 + S2 * P2 = S3 * P3
 * => P3 = (S1 * P1 + S2 * P2) / (S1 + S2)
 * => P3 = (415 * 23.65 + 138 * 16.50) / 553
 * => P3 = 21.8657
 **********************************************************************/
/*pub fn calculate_average_price -> (int a_nargs, ...)
{
    register int l_i;
    va_list l_ap;
    SharesPrice l_current;
    double l_denominator, l_numerator;

    va_start(l_ap, a_nargs);
    l_denominator = 0.0;
    l_numerator = 0.0;
    for (l_i = 0; l_i < a_nargs; l_i++)
    {
         l_current = va_arg(l_ap, SharesPrice);
         l_denominator += l_current.sp_shares * l_current.sp_price;
         l_numerator += l_current.sp_shares;
    }
    va_end(l_ap);
    return (double)(l_denominator / l_numerator);
}*/

/**********************************************************************
 * calculate_percentage_of:
 * Calculate what percentage a_value is from a_from_value.
 **********************************************************************/
pub fn calculate_percentage_of(a_value: f64, a_from_value: f64) -> f64
{
    a_value / a_from_value * 100.0
}

/**********************************************************************
 * convert_from_orig:
 * Returns a price, with an exchange rate applied to it.
 * Used to convert a given currency to a new currency.
 **********************************************************************/
/*double convert_from_orig(double a_price, double a_exchange_rate)
{
    return a_price * a_exchange_rate;
}*/

/**********************************************************************
 * convert_to_orig:
 * Returns a price in the original currency, with the
 * exchange rate no longer applied to it.
 **********************************************************************/
/*double convert_to_orig(double a_converted_price, double a_exchange_rate)
{
    return a_converted_price / a_exchange_rate;
}*/

// Before trade

/**********************************************************************
 * calculate_shares_recommended:
 * Calculates the recommended amount of shares you can buy.
 **********************************************************************/
/*int calculate_shares_recommended(double a_pool, double a_commission, double a_tax, double a_price)
{
       // Note: The int typecast performs truncation. It's better to buy a contract less, than
       // to buy a contract too much. So this truncation provides extra safety and is
       // indeed what we want.
       return (int)((a_pool - (a_tax / 100.0 * a_pool) - a_commission) / a_price);
}*/

/**********************************************************************
 * calculate_leveraged_contracts:
 * Calculates the number of contracts to buy, according to an algorithm
 * that determines an ideal amount of leverage.
 **********************************************************************/
/*int calculate_leveraged_contracts(int a_n)
{
    return ceil(a_n / 3.0) - 1 + a_n;
}*/

/**********************************************************************
 * calculate_stoploss:
 * Calculates the stoploss.
 * Note:
 * Long
 * ----
 * amount selling at stoploss - amount at buying = initial risk of pool
 * (S.Pb + S.Pb.T + C) - (S.Ps - S.Ps.T - C) = R/100 * pool
 * Short
 * -----
 * amount selling - amount buying at stoploss = initial risk of pool
 * (S.Psl + S.Psl.T + C) - (S.Ps - S.Ps.T - C) = R/100 * pool
 **********************************************************************/
/*double calculate_stoploss(double a_price, int a_shares, double a_tax, double a_commission, double a_risk, double a_pool, int a_is_long)
{
    double l_numerator = 0.0;
    double l_denominator = 0.0;
    if (a_is_long == 1)
    {
        l_numerator = a_shares * a_price * (1.0 + a_tax / 100.0) - a_risk / 100.0 * a_pool + 2.0 * a_commission;
        l_denominator = a_shares * 1.0 - a_tax / 100.0;
    }
    else
    {
        l_numerator = a_risk / 100.0 * a_pool + a_shares * a_price * (1.0 - a_tax / 100.0) - 2.0 * a_commission;
        l_denominator = a_shares * 1.0 + a_tax / 100.0;
    }
    return l_numerator / l_denominator;
}*/

/**********************************************************************
 * calculate_risk_input:
 * Calculates the risk based on total pool and input.
 * Consider this the theoretical risk we want to take.
 **********************************************************************/
/*double calculate_risk_input(double a_pool, double a_risk)
{
    return a_risk / 100.0 * a_pool;
}*/

/**********************************************************************
 * calcculate_risk_initial:
 * Calculates the initial risk.
 * This is the risk we will take if our stoploss is reached.
 * This should be equal to the risk_input if everything was
 * correctly calculated.
 * Note:
 * Long
 * ----
 * S.Pb + S.Pb.T + C - (S.Psl - S.Psl.T - C)
 * Short
 * -----
 * S.Ps + S.Psl.T + C - (S.Ps - S.Ps.T - C)
 **********************************************************************/
/*double calculate_risk_initial(double a_price, int a_shares, double a_tax, double a_commission, double a_stoploss, int a_is_long)
{
    if (a_is_long)
        return a_shares * a_price * (1.0 + a_tax / 100.0) - a_shares * a_stoploss * (1.0 - a_tax / 100.0) + 2.0 * a_commission;
    else
        return a_shares * a_stoploss * (1.0 + a_tax / 100.0) - a_shares * a_price * (1.0 - a_tax / 100.0) + 2.0 * a_commission;
}*/

/**********************************************************************
 * calculate_amount:
 * Calculates the amount without tax and commission.
 **********************************************************************/
/*double calculate_amount(double a_price, int a_shares)
{
    return a_price * a_shares;
}*/

/**********************************************************************
 * calculate_amount_with_tax_and_commission:
 * Calculates the amount, including tax and commission.
 * Note:
 * -----
 * AMT = SP + SPT + C (buy)
 * AMT = SP - SPT - C (sell)
 **********************************************************************/
/*double calculate_amount_with_tax_and_commission(double a_price, int a_shares, double a_tax, double a_commission, transaction_type_t a_transaction_type)
{
    if (a_transaction_type == BUY)
        return a_shares * a_price + a_shares * a_price * a_tax + a_commission;
    else
        return a_shares * a_price - a_shares * a_price * a_tax - a_commission;
}*/

/**********************************************************************
 * calculate_amount_with_tax:
 * Calculates the amount (buy/sell) with tax included, but not the commission.
 * Note:
 * -----
 * profit_loss = S.P + S.P.T (buy)
 * profit_loss = S.P - S.P.T (sell)
 **********************************************************************/
/*double calculate_amount_with_tax(double a_price, int a_shares, double a_tax, transaction_type_t a_transaction_type)
{
    if (a_transaction_type == BUY)
        return a_shares * a_price * (1.0 - a_tax / 100.0);
    else
        return a_shares * a_price * (1.0 + a_tax / 100.0);
}*/

/**********************************************************************
 * cost_transaction:
 * Cost of transaction (tax and commission)
 **********************************************************************/
/*double cost_transaction(double a_price, int a_shares, double a_tax, double a_commission)
{
    return a_price * a_shares * a_tax / 100.0 + a_commission;
}*/

/**********************************************************************
 * cost_tax:
 * Cost of tax (buy and sell)
 **********************************************************************/
/*double cost_tax(double a_amount, double a_commission, int a_shares, double a_price, transaction_type_t a_transaction_type)
{
  if (a_transaction_type == SELL)
    return - a_amount - a_commission + a_shares * a_price;
  else
    return a_amount - a_shares * a_price - a_commission;
}*/

/**********************************************************************
 * calculate_price:
 * Calculates the price when buying or selling.
 **********************************************************************/
/*double calculate_price(double a_amount, int a_shares, double a_tax, double a_commission, transaction_type_t a_transaction_type)
{
    double l_numerator = 0.0;
    double l_denominator = 0.0;
    
    if (a_transaction_type == BUY)
    {
        l_numerator = a_amount - a_commission;
        l_denominator = (1.0 + a_tax / 100.0) * a_shares;
    }
    else
    {
        l_numerator = a_amount + a_commission;
        l_denominator = (1.0 - a_tax / 100.0) * a_shares;
    }
    return l_numerator / l_denominator;
}*/

// After trade

/**********************************************************************
 * calculate_risk_actual:
 * Calculates the risk we actually took,
 * based on the data in TABLE_TRADE.
 * Note:
 * risk_actual = S.Pb + S.Pb.T + Cb - (S.Ps - S.Ps.T - Cs)
 * Note:
 * -----
 * It's the same for long and short.
 **********************************************************************/
/*double calculate_risk_actual(double a_price_buy, int a_shares_buy, double a_tax_buy,
    double a_commission_buy, double a_price_sell, int a_shares_sell, double a_tax_sell,
    double a_commission_sell, double a_risk_initial, double a_profit_loss)
{
    if (((a_profit_loss < 0.0) && (fabs(a_profit_loss) < a_risk_initial)) || (a_profit_loss >= 0.0))
        return a_risk_initial;
    else
        return a_shares_buy * a_price_buy * (1.0 + a_tax_buy / 100.0) - a_shares_sell * a_price_sell * (1.0 - a_tax_sell / 100.0) + a_commission_buy + a_commission_sell;
}*/

/**********************************************************************
 * calculate_r_multiple:
 * Function to calculate R-multiple.
 **********************************************************************/
/*double calculate_r_multiple(double a_profit_loss, double a_risk_initial)
{
    return a_profit_loss / a_risk_initial;
}*/

/**********************************************************************
 * calculate_cost_total:
 * Function to calculate the total cost associated with the given trade.
 **********************************************************************/
/*double calculate_cost_total(double a_amount_buy, double a_tax_buy, double a_commission_buy,
    double a_amount_sell, double a_tax_sell, double a_commission_sell)
{
    return a_tax_buy / 100.0 * a_amount_buy + a_commission_buy + a_tax_sell / 100.0 * a_amount_sell + a_commission_sell;
}*/

/**********************************************************************
 * calculate_profit_loss:
 * Calculates the profit_loss, without taking tax and commission into account.
 * Note:
 * -----
 * profit_loss = S.Ps - S.Pb
 * => it's the same for long and short
 **********************************************************************/
/*double calculate_profit_loss(double a_price_buy, int a_shares_buy, double a_price_sell, int a_shares_sell)
{
    return a_shares_sell * a_price_sell - a_shares_buy * a_price_buy;
}*/

/**********************************************************************
 * calculate_profit_loss_total:
 * Calculates the total profit_loss.
 * Note:
 * -----
 * profit_loss = S.Ps - S.Ps.T - C - (S.Pb + S.Pb.T + C)
 * => it's the same for long and short
 **********************************************************************/
/*double calculate_profit_loss_total(double a_price_buy, int a_shares_buy, double a_tax_buy, double a_commission_buy,
    double a_price_sell, int a_shares_sell, double a_tax_sell, double a_commission_sell)
{
    return a_shares_sell * a_price_sell * (1.0 - a_tax_sell / 100.0) - a_shares_buy * a_price_buy * (1.0 - a_tax_buy / 100.0) - (a_commission_buy + a_commission_sell);
}*/

/**********************************************************************
 * calculate_cost_other:
 * Calculates other costs based on the difference that remains.
 **********************************************************************/
/*double calculate_cost_other(double a_profit_loss, double a_profit_loss_total, double a_cost_total)
{
    double l_diff_cost_profit = 0.0;
    l_diff_cost_profit = a_profit_loss - a_profit_loss_total - a_cost_total;
    if (fabs(l_diff_cost_profit) > 0.0)
        return l_diff_cost_profit;
    else
      return 0.0;
}*/
