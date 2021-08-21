/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   final_calc.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 14:12:03 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/21 21:54:45 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//The Final will be a sum between the compound interest for the principal and the future value of a series, as below: 
// -------> Final = [ Compound interest for principal ] + [ Future value of a series ]
// -------> Compound interest for principal = P(1+r/n)^(nt)
// -------> Future value of a series = PMT × (((1 + r/n)^(nt) - 1) / (r/n))
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// The resulting mathematical formula is:
//--------> Final = [ P(1+r/n)^(nt) ] + [ PMT × (((1 + r/n)^(nt) - 1) / (r/n)) ]
//Notations:
// P = the principal investment amount (the initial deposit or loan amount)
// r = the annual interest rate (decimal)
// t = the time the money is invested or borrowed for
// n = number of times interest is compounded ( if you make 12 extra payments yearly - one each month - n will be 12);
// PMT = monthly payment

pub fn final_calc(principal:f64, rate:f64, time:f64, pmt:f64, n:f64) => f64
{
	//we make sure that te rate is expressed as a float, not as a percentage; e.g 10% = 10 / 100 = 0.1;
	rate = rate / 100.0;
	//the formula to calculate the compound interest for the principal is:
	//Compound interest for principal = P * (1+r/n)^(nt)
	//first we calculate the power part of the formula: ( 1 + r / n) ^ (n * t)
    let power = f64::powf(1.0 + rate / 12.0, time * 12.0);
	//now we multiply the result with the principal so we get the compound interest for the principal:
    let ci = principal * power;
	//the formula to calculate the future value of a series is: PMT × (((1 + r/n)^(nt) - 1) / (r/n));
	//we calculate the power part of the formula first: (1 + r / n)^(n * t)
    let future_value_power = f64::powf(1.0 + (rate / 12.0), 12.0 * time);
	// we use the result inside the original formula and proceed to calculate the future value:
    let future_value = pmt * ((future_value_power - 1.0) / (rate / 12.0));
	//the final will be a sum of the compound interest for the principal and the future value series:
	//Final = [ Compound interest for principal ] + [ Future value of a series ]:
    let final: f64 = ci + future_value;
	//returning final
	final
}