use std::io;
use std::env;
#[derive(Debug,Clone)]
enum Defrd{
	mtSub,
	aSub(String, Box<FAE>, Box<Defrd>),		
}

fn lookup(name : String, saved : Defrd)->FAE{
	match saved{
		Defrd::mtSub =>{
				println!("look up {}: is mt",name);
				FAE::id(name)
				
		},
		Defrd::aSub(i, v, saved) =>{
			if i.eq(&name){
					println!("look up {}: is {:?}",name,*v);
					match *v{
						FAE::closureV(s,b,d) =>{
							FAE::closureV(s,b,Box::new(*saved.clone()) )		
						}	,
						_ =>{*v},
					}
			}
			else{
				lookup(name,*saved)		
			}		
		}
	}		
}

#[derive(Clone,Debug)]
enum FAE{
	num (f32),
	add (Box<FAE>, Box<FAE>),
	sub ( Box<FAE>,  Box<FAE>),
//	with ((String,Box<FAE>), Box<FAE>),
	id (String),
	fun (String, Box<FAE>),
	app (Box<FAE>, Box<FAE>),
	numV(f32),
	closureV(String, Box<FAE>, Box<Defrd>),		
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
				v.push('{') ;	
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
					
					if c == 'f'{
						if s.len() > count+3 && "fun".eq(&s[count..count+3]) {
							if ind_instance.oper==' ' &&v.len()==1{
									ind_instance.oper='f';
							}
							skip=2;
							count+=1;
							continue;
						}
					}
					else if c=='w' {
						if s.len() > count+4 && "with".eq(&s[count..count+4])  {
							if ind_instance.oper==' ' && v.len()==1{
									ind_instance.oper='w';
							}
							skip=3;
							count+=1;
							continue;
						}
					}

					if ind_instance.oper=='w' && ind_instance.id ==0{
							ind_instance.id = count;
					}
					else if ind_instance.oper =='f' && ind_instance.id==0{
						ind_instance.id = count;		
					}
					else if v.len()==1 /* && (ind_instance.oper=='-' ||ind_instance.oper=='+')*/ {
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
/*
fn subst(wae : FAE, idtf : String, val :f32) -> FAE{
	println!("subst ae: {:?} ,id: {:?}, val {}",wae, idtf, val);
	match wae{
		FAE::num(_) => 	{wae },
		FAE::add(l,r) =>  { FAE::add(Box::new(subst(*l,idtf.clone(),val )),Box::new(subst(*r,idtf,val ) ) ) },
		FAE::sub(l,r) =>  { FAE::sub(Box::new(subst(*l,idtf.clone(),val )),Box::new(subst(*r,idtf,val ) ) ) },
		FAE::with((i,v),e) => {
			if i.eq(&idtf){
				{  FAE::with((i, Box::new(subst(*v, idtf, val ) )),e)  }		
			}	
			else{
				{  FAE::with((i, Box::new(subst(*v, idtf.clone(), val ) )), Box::new(subst(*e, idtf, val )) ) }		
			}
		},
		FAE::id(s) =>{
			if s.eq(&idtf){
				{FAE::num(val) }		
			}
			else{
				{FAE::id(s) }		
			}
		},
	}

}
*/
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
	if ind_instance.rhs_start !=0 && ind_instance.lhs_start !=0 && ind_instance.oper==' ' {
		ind_instance.oper = 'a';
	}

	if ind_instance.rhs_start ==0 && ind_instance.oper=='-'{
		ind_instance.oper = ' ';		
	}
	let subs_lhs = String::from(&s[ind_instance.lhs_start..ind_instance.lhs_end+1]);
	let subs_rhs = String::from(&s[ind_instance.rhs_start..ind_instance.rhs_end+1]);	
	let subs_id = String::from(&s[ind_instance.id..ind_instance.id+1]);	
	let subs_id_expr = String::from(&s[ind_instance.id_expr_start..ind_instance.id_expr_end+1]);	
//	println!("where is id {} and {}",ind_instance.id, ind_instance.oper);
	{(subs_lhs, subs_rhs, subs_id, subs_id_expr)}
}

fn parse (s : String)  -> FAE{
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
	println!("{}:{}:{}:{}",subs_lhs, subs_rhs, subs_id, ind_instance.oper);
	
	match ind_instance.oper {
		' ' =>FAE::num(subs_lhs.parse::<f32>().unwrap() ),
		'+' =>FAE::add( Box::new(parse(subs_lhs)), Box::new(parse(subs_rhs))),
		'-' =>FAE::sub(Box::new(parse(subs_lhs)), Box::new(parse(subs_rhs))),
		'w' =>{	
			println!("app (({},{}),{}",	subs_id, subs_rhs, subs_id_expr);
			FAE::app(Box::new(  FAE::fun(subs_id.to_string(), Box::new(parse(subs_rhs)))), Box::new(parse(subs_id_expr)) )
		},
		'd' =>FAE::id(subs_id.to_string()),
		'f' =>FAE::fun(subs_id.to_string(), Box::new(parse(subs_rhs))),
		'a' =>FAE::app(Box::new(parse(subs_lhs)), Box::new(parse(subs_rhs))),
		_ => {panic!("err")},		
	}
	
}


fn num_minus(l :FAE, r: FAE)->FAE{
		let mut ret:f32 = 0.0;

		if let FAE::numV(d) = l{
			if let FAE::numV(s) = r{
				ret = d-s;
				{ return  FAE::numV(ret);} 
			}
			else{
					
				return FAE::add(Box::new(l),Box::new(r));		
			}
		}
		else{
			return FAE::add(Box::new(l),Box::new(r));		
		}
		

}

fn num_plus(l :FAE, r: FAE)->FAE{
		let mut ret:f32 = 0.0;

		if let FAE::numV(d) = l{
			if let FAE::numV(s) = r{
				ret = d+s;
				{ return  FAE::numV(ret);} 
			}
			else{
					
				return FAE::add(Box::new(l),Box::new(r));		
			}
		}
		else{
			return FAE::add(Box::new(l),Box::new(r));		
		}
		

}
fn interp(ae : FAE, ds: Defrd) ->FAE{
	match ae {
		FAE::num(n) => {FAE::numV(n)},
		FAE::add(l,r) =>{
			{
					
					println!("{:?} + {:?}",*l,*r);
					num_plus(interp(*l,ds.clone()), interp(*r,ds))	}
		},
		FAE::sub(l,r) =>{
			{ num_minus(interp(*l,ds.clone()) , interp(*r,ds))	}
		},
//		FAE::with((i,v),e) =>{
//			{interp(subst(*e,i,interp(*v)) ) }		
//		},
		FAE::id(s) =>{  
				lookup(s,ds)
		},
		
		FAE::fun(p,b) =>{
			FAE::closureV(p, b, Box::new(ds))
		},
		FAE::app(f,a)=>{
			let f_val = interp(*f, ds.clone());
			println!("f_val  {:?}\n",f_val);
			let a_val = interp(*a, ds);
			println!("a_val  {:?}\n",a_val);
			let mut body= FAE::num(0.0);
			let mut param = String::new();
			let mut saved_ds =Box::new(Defrd::mtSub) ;
			if let FAE::closureV(p,b,d) = f_val{
				body =*b;
				param = p;	
				saved_ds = d;	
			}

			println!("interp {:?}   {:?} \n",body, Defrd::aSub(param.clone(),Box::new(a_val.clone()), saved_ds.clone() ));
			interp(body, Defrd::aSub(param,Box::new(a_val), saved_ds )  ) 
		},
		_ => {
			panic!("interp wrong input");	
			}

			
	}
}
fn main() {
	let mut parse_flag = 0;
	let option : Vec<String> =  env::args().collect();
	if option.len()>2&& option[1].eq("-p"){
		parse_flag = 1;
	}	
	while(true){
			let mut inn :String = String::new();
			io::stdin().read_line(&mut inn)
			.expect("Failed to read line");
			let a = parse(String::from(inn));
			println!("{:?}",a);
			let mut ds = Defrd::mtSub;
			println!("{:?}", interp(a,ds));
	}


}
