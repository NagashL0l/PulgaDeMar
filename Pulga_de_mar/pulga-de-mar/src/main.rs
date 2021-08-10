

struct Pulga{
    id: usize, // empezando en el 1 (el 0 se dejará sin asignar)
    edad: u8, // edad en días
    size: u8, // en mm (de 15 a 25mm)
    nivel_hambre: u8 // del 1 al 5 donde 1 es nada y 5 es mucho
}

impl Pulga {
    fn nacimiento(pulgas: Vec<Pulga>) -> Pulga{ //constructor
        Pulga{
            id: pulgas.len() + 1,
            edad: 0,
            size: 15,
            nivel_hambre: 5
        }
    }   

    fn muerte(pulga: Pulga){

    }      
}

fn main() {
    let mut pulgas: Vec<Pulga> = Vec::new();

    let senilla = Pulga{
        id: 0,
        edad: 10,
        size: 20,
        nivel_hambre: 5
    };
    pulgas.push(semilla);
    println!("ID de la pulga origen: {}",pulgas[0].id);
    println!("edad de la pulga origen: {} días",pulgas[0].edad);
    
    let mut p1 = Pulga::nacimiento(pulgas);
    pulgas.push(p1);
    let mut p2 = Pulga::nacimiento(pulgas);
    pulgas.push(p2);
    let mut p3 = Pulga::nacimiento(pulgas);
    pulgas.push(p3);
    let mut p4 = Pulga::nacimiento(pulgas);
    pulgas.push(p4);
    let mut p5 = Pulga::nacimiento(pulgas);
    pulgas.push(p5);
    

}
