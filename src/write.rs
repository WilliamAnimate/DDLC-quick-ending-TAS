// use std::fs::File;
// use std::io::Write;

use std::env;
use std::path::Path;

fn main() {
	// Retrieve the value of the appdata environment variable
	if let Ok(appdata) = env::var("appdata") {
		// Construct the path
		let full_path = Path::new(&appdata).join("RenPy").join("DDLC-1454445547").join("persistent");

		// Check if the file or directory exists
		if full_path.exists() {
			println!("The file or directory exists.");
		} else {
			println!("The file or directory does not exist.");
		}
	} else {
		println!("Unable to retrieve the value of appdata.");
	}
}
	// let bin = "-----BEGIN CERTIFICATE-----
	// eJyVVltzHEcVlhXJlkaSZckyweCEtZI469jZleTYsQOJCRGYaO3FOGkYB5tmLr3T
	// Y81Mz5mekbyBIRdYa5Zrwv0a7vc/QBW/gCoeeKCo4hfAE/DCK8Xpnt2dNak8RKra
	// 6ulz+pzv3L7ulyedhEVxtxGzRPoyZVFqXK2WsO/kKzCZwz11MhcHVjflicg8DlOt
	// CXKISsYimiZWJAMrZRKmHUrtzA9SP6LUkAzP778JB+rmkYmJCYnnJZfoDg+dOXfW
	// ZefOd2DmzTL7/IWz7gV7HWZZ7xoYZMploYC5vnaY0jhhHYaYHXQ4XxCj4ycSdwUL
	// YaFNDlCHC8kiOJjDonkCbW8+cWNTOFmI4cgbm5uXn9E/Tc8KWbP020jiLhza+s8f
	// Tl1vHd+DJXMGjz1bszyE07gDy4UkR6jlcJ/tMGUGIQiUSQmHczJrRanvcGalsIJJ
	// mQtF5G9bdNsPAjjSJrN4IKO2R0N4R5vMOAGzEgtF97bJIZ0+RwQikSX+d7bJEuJH
	// DSdlCd0RAcKGo8MKYfZFZFxDFElq2QHb9B3M8LuwQu/O4ZhNDNrxA0Zjy2Nwn7kP
	// g1gnM8pe5DEX7s/hPXU+denpv60t3Prn76/xab2Ee//1Bp+rduer5cFK4XC1u1It
	// j1bL+6rl/ZiGhfF8SaiNpGS+TFAqYt+RcLwSHB6rLHVZx8qCFFYr+TKVaVeFN1b/
	// ByrxXNmNVub6Ah7U+y+d/8t/ycKgIAkLhOXCQ9UJQzU0SyJsBDgxBrDMYkcELkvg
	// 4UqwWKWX6kP1SnaQ+lZM4yzBbEsEdrISrdAsdnE8KLZaitVgzjZW45FKYUHGzPGt
	// QPeAhFMV9tlulvhlK52u9GfLjk+yCB4dVYgYiHdHDa6IoFEpT+uOg+Z4fOMZXBvD
	// QXf8JM0QiPRfZLA+HvkghKGDjfG87wgfKxZmKYMz4250PfwQ0yXhsSoooxSoLoaz
	// 1fY0Eo4VwrmRhWw42rpuj7dHo2Vbzjacb/Ma338TCeICP/5WA3IZWcyAJ3BA3pvD
	// +2y+ytWwPFnXA56y21iOWMJTrQ3TUCTUuT0cuYuXLv5jQv9pVasT0tTH/fe3Fs15
	// 3Agz6TtD5adHylkOH7D5A9rLMzls2vzBHD5YN5dQ5mV+Ez00uUB0DeF58KHCXB4T
	// SBYwJ9WSS4W5ghLbC5tSl59iDYOuln24yMi8zqBE3tnGJD7b5g+1JvgJU0Eghof8
	// l1JFO7DVJkaa+MylctuPodXmD6NiXeO7nMMVm8yWtehiidptflJLPpLDVZs/otcf
	// zeGaTRa11pjh59r8FFd5fb7emm0ZrSmWA7H5owU/jQ4a5rTingb+w8d4Uyt+vN4f
	// /aGyafO1YdWqbjSujq3hOh57IYdP1FW8QUf3mR95cKNP5lQ8NIsULriJNwPeXh2R
	// hJbiDryJUvhkQebvamd6ZWL6yp8n78CnyJTuVSsHu66R6gYGp68/dGnB7Zv3lC0B
	// rJ+RhZh3cXtoqzO05REjFJnE7seqAi/ISihsRRKJCALVp6jvMvDJAdeXqiPhFpnB
	// +JBZsHDbZMrKUgEBWVQdZnUU6TsBVhXCPlnZwaOiHCDasUp7EPXJ/luiG1oxiBxi
	// nJEsCMqLE6BPjrhMbiO3/h+AhN8ic/qi9lOcXwmyNUkM5ZRFGlbaJzMB3hOZujuy
	// Nlkoh1pmMrX8CHb6xBgbj91Rx5MlycUuZWGcdumuH7liF25j6u8akG6lvoiayJCY
	// xAFTv4jFjC13gMOFT/fJsi7uIBtc4ZDwmT45UFqTkOfw2Tp3Ll38d2mUuyP7nFWz
	// SA4hhsijY1G+hM+FXctPS8qClwtkryGnUQqvtJbJzIgWXm1tkJnR5H+utZjZfH1Q
	// eL6hp+PzOfRsfuat6Oc59QC6g3p7ORSo+FgO/bLjdrmfsh58oTAn1dcGfLEHXyrM
	// x/HjefRfX22L3Zqy0xURqzlWVLNZjVsxelk9XdP34JOr0uoKvB4U4NWT8OUefKUw
	// j6GF1ZJzm453N3vEkbcKX+3Ba4U5pfhPyI0evF5oQJHwJYOv9eDrhW58FPbgG+Ua
	// 2xW+2YNvlaTkiKjjew0v0HNWtmcPvl2asQPVo9/pwXeLjJ/N4Xt18yhuU5ycgGoi
	// H9zDVD2/6Dp8vzBPowKyIr7GUhE2Lbupszn4dUSIrN9cW/MC5U691Lb+fvwp88r0
	// vj34QemUQuan8MOCLxVv58H3xtZfl//4Quu1PfhRwQcfr+/BjwtN+XQQIPykMA+q
	// b5YkIsGnlBsoDvppoe8B5GArSfFXJAx+VuZVo/n524Gyldl/ut46tge/KPRV8aZE
	// wS8Lc//IHfyq4IPXam0Pfl2YC1o09oSG34zgqRcTPlVZCr8t9+7S+x2W6Vw/sxv/
	// A+jrV1g=
	// -----END CERTIFICATE-----";
	// // write file then move to appdata

	// let mut file = File::create("persistent").expect("Failed to create file");
	// file.write_all(bin.as_bytes()).expect("Failed to write to file");