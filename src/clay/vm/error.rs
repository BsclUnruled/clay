pub fn throw(message: &str)->!{
    panic!("{}",message)
}

pub fn set_unsetable(type_name: &str, property_name: &str)->!{
    throw(&format!("Error(set_unsetable):尝试设置{}的属性{}", property_name, type_name))
}

pub fn get_ungetable(type_name: &str, property_name: &str)->!{
    throw(&format!("Error(get_ungetable):尝试获取{}的属性{}", property_name, type_name))
}

pub fn set_undef(property_name: &str)->!{
    set_unsetable("undef", property_name)
}