fn main() {
    let rows = 5;
    let columns = 3;
    let mut field = vec![vec![0; columns]; rows];
    print(&field);
}

fn print(field: &Vec<Vec<i32>>) {
    for row in field {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}
