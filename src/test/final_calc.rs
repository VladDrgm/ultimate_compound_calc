/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   final_calc.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 14:12:03 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/26 17:43:22 by vdragomi         ###   ########.fr       */
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
// n = number of times interest is compounded;
// PMT = monthly payment

pub fn final_calc(principal:f64, mut rate:f64, time:f64, n:f64) -> f64
{
	//we make sure that te rate is expressed as a float, not as a percentage; e.g 10% = 10 / 100 = 0.1;
	rate = rate / 100.0;
	//the formula to calculate the compound interest for the principal is:
	//Compound interest for principal (ci) = P * (1+r/n)^(nt)
	//first we calculate the power part of the formula: ( 1 + r / n) ^ (n * t)
    let power = f64::powf(1.0 + rate / n, time * n);
	//now we multiply the result with the principal so we get the compound interest for the principal:
    let ci = principal * power;
	//return ci
	ci
}
