use std::io;
use std::env;
#[derive(Clone,Debug)]
enum AE{
	num (f32),
	add (Box<AE>, Box<AE>),
	sub ( Box<AE>,  Box<AE>),
	with ((String,Box<AE>), Box<AE>),
	id (String),
}
#[derive(Debug)]
struct s_index{
	lhs_start : usize,
	lhs_end : usize,
	id_expr_start: usize,
	id_expr_end:usize,
	id: usize,
	rhs_start : usize,
	rhs_end : usize,
	oper : char,
}

fn subst_index(s: &String, ind_instance : &mut s_index){
	let mut count : usize = 0;
	let mut v = Vec::new();
	let mut lhs_write = 0;
	let mut skip=0;
	for i in s.chars(){
		if skip>0{
			skip= skip-1;
			count+=1;
			continue;		
		}	
		match i {
			'w' => {
					if s.len() > count+4 && "with".eq(&s[count..count+4]){
							ind_instance.oper='w';
							skip=3;
					}
			},	
			
			'{' => {
				if v.len() ==1 {
					if ind_instance.lhs_start ==0{
						ind_instance.lhs_start = count;
					}
					else{
						ind_instance.rhs_start = count;
					}
				}
				else if v.len()==2 {
					if ind_instance.id_expr_start ==0 && ind_instance.oper =='w'{
						ind_instance.id_expr_start = count;		
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
				else if v.len()==2{

					if ind_instance.id_expr_end ==0 && ind_instance.oper=='w'{
						ind_instance.id_expr_end =count;		
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
					else if v.len()==2{
						
						with_name_expr(&s,  ind_instance,count);		
					}

				}
				else if c ==' '{
					if lhs_write == 1 {
							lhs_write =2; //lhs writing done
					}		
				}
				else if (c>='a' && c<='z') || (c>='A' && c<='Z') {
					if v.len()==2 && ind_instance.id ==0{
							ind_instance.id = count;
					}
					else if v.len()==1{
						if lhs_write!= 2{
							ind_instance.lhs_start = count;
							ind_instance.lhs_end = count;
							lhs_write=2;
						}
						else{
							ind_instance.rhs_start = count;
							ind_instance.rhs_end = count;
						}
					}
					else if v.len()==0{
						ind_instance.id = count;
						ind_instance.oper ='d';		
					}
				}
			},
		}

		count+=1;
	}	
}
fn subst(wae : AE, idtf : String, val :f32) -> AE{
	match wae{
		AE::num(_) => 	{wae },
		AE::add(l,r) =>  { AE::add(Box::new(subst(*l,idtf.clone(),val )),Box::new(subst(*r,idtf,val ) ) ) },
		AE::sub(l,r) =>  { AE::sub(Box::new(subst(*l,idtf.clone(),val )),Box::new(subst(*r,idtf,val ) ) ) },
		AE::with((i,v),e) => {
			if i.eq(&idtf){
				{  AE::with((i, Box::new(subst(*v, idtf, val ) )),e)  }		
			}	
			else{
				{  AE::with((i, Box::new(subst(*v, idtf.clone(), val ) )), Box::new(subst(*e, idtf, val )) ) }		
			}
		},
		AE::id(s) =>{
			if s.eq(&idtf){
				{AE::num(val) }		
			}
			else{
				{AE::id(s) }		
			}
		},
	}

}
fn with_name_expr (s : &String,ind_instance : &mut s_index, count : usize) {
		if ind_instance.oper =='w' && ind_instance.id_expr_start ==0{
			ind_instance.id_expr_start =count;
			if count >0 && s.as_bytes()[count-1] as char =='-'{
					ind_instance.id_expr_start-=1;			
			}
			ind_instance.id_expr_end =count;
			let mut while_idx: usize=count+1;
			while(s.len()>while_idx){
				let c = s.as_bytes()[while_idx] as char ;
				if (c >= '0' && c<='9')|| c=='.'{
					ind_instance.id_expr_end  = while_idx;
				}
				else{
					break;		
				}
				while_idx +=1;
			}
		}
}
fn substring(ind_instance : &mut s_index, s : &String)->(String, String,String,String){
	if ind_instance.rhs_start ==0 && ind_instance.oper=='-'{
		ind_instance.oper = ' ';		
	}
	let subs_lhs = String::from(&s[ind_instance.lhs_start..ind_instance.lhs_end+1]);
	let subs_rhs = String::from(&s[ind_instance.rhs_start..ind_instance.rhs_end+1]);	
	let subs_id = String::from(&s[ind_instance.id..ind_instance.id+1]);	
	let subs_id_expr = String::from(&s[ind_instance.id_expr_start..ind_instance.id_expr_end+1]);	
	{(subs_lhs, subs_rhs, subs_id, subs_id_expr)}
}

fn parse (s : String) -> AE{
	let mut ind_instance = s_index {
		lhs_start :0,
		lhs_end:0,
		id_expr_start :0,
		id_expr_end : 0,
		id:0,
		rhs_start : 0,
		rhs_end:0,
		oper : ' ',
	};

	subst_index(&s, &mut ind_instance);
	let (subs_lhs, subs_rhs, subs_id, subs_id_expr) = substring(&mut ind_instance, &s);
	match ind_instance.oper {
		' ' =>AE::num(subs_lhs.parse::<f32>().unwrap() ),
		'+' =>AE::add( Box::new(parse(subs_lhs)), Box::new(parse(subs_rhs))),
		'-' =>AE::sub(Box::new(parse(subs_lhs)), Box::new(parse(subs_rhs))),
		'w' =>AE::with(( subs_id ,Box::new(parse(subs_id_expr ) )), Box::new(parse(subs_rhs))),
		'd' =>AE::id(subs_id.to_string()),
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
		AE::with((i,v),e) =>{
			{interp(subst(*e,i,interp(*v)) ) }		
		},
		AE::id(s) =>{ panic!("err") },
			
	}
}
#[derive(Debug)]
enum a {
	ab(String),	
		}
fn main() {
		let mut parse_flag = 0;
		let option : Vec<String> =  env::args().collect();
		if option.len()>2 && option[1].eq("-p"){
			parse_flag = 1;
		}
													 
	while(true){
			let mut inn :String = String::new();
			io::stdin().read_line(&mut inn)
			.expect("Failed to read line");
			let a = parse(String::from(inn));
			if parse_flag ==1{
					println!("{:?}",a);
				}
			println!("{}", interp(a));
	}


}
