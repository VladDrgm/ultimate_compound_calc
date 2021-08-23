/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   test.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/21 22:11:45 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/21 22:25:52 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod final_calc;

fn main()
{
	if final_calc::final_calc(5000.0, 5.0, 10.0, 100.0, 12.0) == 23763.27543301812
	{
		println!("calculations are correct");
	}
	else
	{
		println!("oops, incorrect math!")
	}
}