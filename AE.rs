use std::io;
#[derive(Debug)]
enum AE{
	num (f32),
	add (Box<AE>, Box<AE>),
	sub ( Box<AE>,  Box<AE>),
}
#[derive(Debug)]
struct s_index{
	lhs_start : usize,
	lhs_end : usize,
	rhs_start : usize,
	rhs_end : usize,
	oper : char,
}
fn subst_index(s: &String, ind_instance : &mut s_index){
	let mut count = 0;
	let mut v = Vec::new();
	let mut lhs_write = 0;
	for i in s.chars(){
		match i {
			'{' => {
				if v.len() ==1 {
					if ind_instance.lhs_start ==0{
						ind_instance.lhs_start = count;
					}
					else{
						ind_instance.rhs_start = count;
					}
				}
				v.push('{')  ;	
				
			},

			'}' => {
				v.pop();
				if v.len() ==1 {
					if ind_instance.lhs_end ==0{
						ind_instance.lhs_end = count;
						lhs_write=2;
					}
					else{
						ind_instance.rhs_end = count;
					}
				}	
			},
			(c)  => {
				if v.len()==1 && ind_instance.oper == ' ' &&(c=='+' || c =='-'){
					ind_instance.oper = c;
				}
				else if  (c >= '0' && c<='9') || c =='.' {
					if v.len()<=1{
						if lhs_write !=2 {
							if ind_instance.lhs_start ==0 && lhs_write==0{
									ind_instance.lhs_start = count;
							}
							ind_instance.lhs_end = count;
							if count >0 && s.as_bytes()[count-1] as char =='-'{
								ind_instance.lhs_start-=1;			
							} 
							lhs_write = 1;
						}
						else {
							if ind_instance.rhs_start ==0{
									ind_instance.rhs_start = count;
							}
							ind_instance.rhs_end = count;
							if count >0 && s.as_bytes()[count-1] as char =='-'{
								ind_instance.rhs_start-=1;			
							}
						}
					}

				}
				else if c ==' '{
					if lhs_write == 1 {
							lhs_write =2; //lhs writing done
					}		
				}

				
			},
		}

		count+=1;
	}	
}

fn substring(ind_instance : &mut s_index, s : &String)->(String, String){
	if ind_instance.rhs_start ==0{
		ind_instance.oper = ' ';		
	}
	let subs_lhs = String::from(&s[ind_instance.lhs_start..ind_instance.lhs_end+1]);
	let subs_rhs = String::from(&s[ind_instance.rhs_start..ind_instance.rhs_end+1]);	
	{(subs_lhs, subs_rhs)}
}

fn parse (s : String) -> AE{
	let mut ind_instance = s_index {
		lhs_start :0,
		lhs_end:0,
		rhs_start : 0,
		rhs_end:0,
		oper : ' ',
	};

	subst_index(&s, &mut ind_instance);
	let (subs_lhs, subs_rhs) = substring(&mut ind_instance, &s);
	
	match ind_instance.oper {
		' ' =>AE::num(subs_lhs.parse::<f32>().unwrap() ),
		'+' =>AE::add( Box::new(parse(subs_lhs)), Box::new(parse(subs_rhs))),
		'-' =>AE::sub(Box::new(parse(subs_lhs)), Box::new(parse(subs_rhs))),
		_ => {panic!("err")},		
	}
	
}
fn interp(ae : AE) ->f32{
	match ae {
		AE::num(n) => n,
		AE::add(l,r) =>{
			{interp(*l)+ interp(*r)	}
		},
		AE::sub(l,r) =>{
			{interp(*l) - interp(*r)	}
		},
			
	}
}
fn main() {

		while(true){
			let mut inn :String = String::new();
			io::stdin().read_line(&mut inn)
			        .expect("Failed to read line");
			let a = parse(String::from(inn));
	
			println!("{}", interp(a));
		}

}
