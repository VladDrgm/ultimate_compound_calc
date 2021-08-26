/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   rate_calc.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 14:12:54 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/26 18:08:14 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//the formula for interest rate is:
//r = n[(A / P) ^ 1 / (n * t) - 1]


pub fn rate_calc(principal:f64, total:f64, time:f64, n:f64) -> f64
{
	let power = f64::powf(total / principal, 1.0 / (n * time));
	let rate:f64 = n * (power - 1.0);
	rate * 100.0
}