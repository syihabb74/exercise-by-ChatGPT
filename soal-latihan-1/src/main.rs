extern crate chrono;
use chrono::prelude::*;


fn main() {
    let mut library = Library::new();

    library.add_book(String::from("The Rust Programming Language"), String::from("Steve"), 2018, 3);
    library.add_book(String::from("Programming Rust"), String::from("Jim Blandy"), 2017, 2);

    library.add_member(String::from("Alice"));
    library.add_member(String::from("Bob"));

    match library.borrow_book(1, String::from("The Rust Programming Language")) {
        BorrowingStatus::Success => println!("Peminjaman berhasil"),
        BorrowingStatus::Failed(err) => println!("Peminjaman gagal: {}", err),
    }

    match library.borrow_book(1, String::from("Nonexistent Book")) {
        BorrowingStatus::Success => println!("Peminjaman berhasil"),
        BorrowingStatus::Failed(err) => println!("Peminjaman gagal: {}", err),
    }

}


#[derive(Debug)]
struct Book {
    title : String,
    author : String,
    year : u32,
    copies : u32
}

#[derive(Debug)]
struct Member {
    name : String,
    id : u32,
    borrowed_books : Vec<BorrowTransaction>
}

impl Member {
    fn return_book (&mut self, title : &String, library : &mut Library ) {
        for mut book in &mut library.books {
            if &book.title == title {
                book.copies += 1;
                return;
            }
        }
        println!("Buku tidak ditemukan");
    }
}


#[derive(Debug)]
struct BorrowTransaction {
        member_id : u32,
        title : String,
        borrow_date : String
}

#[derive(Debug)]
struct Library {
    books : Vec<Book>,
    members : Vec<Member>,
    transactions : Vec<BorrowTransaction>
}

impl Library {
    fn new () -> Library {
        Library {
            books : Vec::new(),
            members : Vec::new(),
            transactions : Vec::new()
        }
    }
    fn add_book(&mut self, title : String, author : String, year : u32, copies : u32) {

        if self.books.len() < 1 {
            let book : Book = Book {
                title,
                author,
                year,
                copies
            };
            self.books.push(book);
            return;
        }

        for b in &self.books {
            if b.title == title {
                println!("Buku yang didaftarkan ke perpustakaan sudah ada silahkan daftarkan buku lain!!");
                return;
            };
        }

        let book : Book = Book {
            title,
            author,
            year,
            copies
        };

        self.books.push(book);

    }

    fn add_member (&mut self, name : String) {

        if self.members.len() < 1 {
           let member : Member = Member {
               name,
               id : self.members.len() as u32 + 1,
               borrowed_books : Vec::new()
           };
            self.members.push(member);
            return;
        }

        let member : Member = Member {
            name,
            id : self.members.len() as u32 + 1,
            borrowed_books : Vec::new()
        };

        for m in &self.members {
            if (m.id == member.id) {
                println!("Jangan mencoba merusak sistem :>");
                return;
            }
            self.members.push(member);
            return;
        }
    }

    fn borrow_book (&mut self, member_id : u32 ,title : String) -> BorrowingStatus {
            if member_id > self.members.len() as u32 {
                return BorrowingStatus::Failed(String::from("Member tidak ditemukan."));
            }

        let is_title_exist = false;
        for book in &mut self.books {
            if book.title == title {
                if book.copies < 0 {
                    return BorrowingStatus::Failed(String::from("Stok buku sudah habis."));
                }
                book.copies -= 1;
                self.transactions.push(BorrowTransaction {
                    member_id : member_id.clone(),
                    title : title.clone(),
                    borrow_date : Local::now().format("%d-%m-%Y").to_string()
                });
                self.members[member_id as usize - 1].borrowed_books.push(BorrowTransaction {
                    member_id : member_id.clone(),
                    title : title.clone(),
                    borrow_date : Local::now().format("%d-%m-%Y").to_string()
                });
                return BorrowingStatus::Success
            }
        }

        return BorrowingStatus::Failed(String::from("Buku tidak ditemukan."));

    }
}

enum BorrowingStatus {
    Success,
    Failed(String)
}


