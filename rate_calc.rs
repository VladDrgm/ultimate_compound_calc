/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rate_calc.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 14:12:54 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/26 17:29:11 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//the formula for interest rate is:
//r = n[(A / P) ^ 1 / (n * t) - 1]


pub fn rate_calc(principal:f64, total:f64, time:f64, pmt:f64, n:f64) => f64
{
	let power = f64:powf(total / principal, 1 / (n* t));
	let rate:f64 = n * (power - 1);
	rate
}