/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 22:11:45 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/26 18:24:09 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod final_calc;
mod principal_calc;
mod rate_calc;
use assert_approx_eq::assert_approx_eq;


//pub fn final_calc(principal:f64, mut rate:f64, time:f64, n:f64) -> f64
fn test1()
{
	if final_calc::final_calc(5000.0, 5.0, 10.0, 12.0) == 8235.0474884514
	{
		println!("Final calculations are correct");
	}
	else
	{
		println!("oops, incorrect math!")
	}
}


//pub fn rate_principal(rate:f64, total:f64, time:f64, n:f64) => f64

fn test2()
{
	if principal_calc::principal_calc(8.0, 10000.0, 5.0, 12.0) == 6712.10444429162
	{
		println!("Principal calculations are correct");
	}
	else
	{
		println!("oops, incorrect math!")
	}
}

//pub fn rate_calc(principal:f64, total:f64, time:f64, n:f64) -> f64

fn test3()
{
	assert_approx_eq!(rate_calc::rate_calc(5000.0, 8235.0474884514, 10.0, 12.0), 5.0);
}

pub fn main()
{
	test1();
	test2();
	test3();
}