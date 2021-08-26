/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   principal_calc.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 14:12:52 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/26 17:33:08 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


//P = A / [(1 + r/n)] ^ n / t

pub fn rate_principal(rate:f64, total:f64, time:f64, pmt:f64, n:f64) => f64
{
	let power = f64:powf(1 + r / n, n / t);
	let principal:f64 = total / power;
	principal
}