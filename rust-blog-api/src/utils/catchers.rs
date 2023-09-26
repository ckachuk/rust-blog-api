use rocket::Request;



#[catch(400)]
pub fn bad_request(req: &Request) -> String {
    println!("{}", req.to_string());
    format!("Solicitud incorrecta en la ruta: '{}'.", req.uri())
}


#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("No se encontró la ruta: '{}'.", req.uri())
}



#[catch(401)]
pub fn unauthorized(req: &Request) -> String {
    format!("Invalid authentication credentials '{}'.", req.uri())
}