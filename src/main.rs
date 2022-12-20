/// the Delay function makes the program wait 300 milliseconds this is done to give the computer time for the input camand to run.
///
fn delay() {
    use std::{thread, time};
    let three_millis = time::Duration::from_millis(300);
    let now = std::time::Instant::now();
    thread::sleep(three_millis);
    assert!(now.elapsed() >= three_millis);
}

fn main() {
    use winput::Mouse;
    use winput::{Button, Vk};
    //These 3 lines open up the run terminal on windows devices
    winput::press(Vk::LeftWin); 
    winput::send(Vk::R); 
    winput::release(Vk::LeftWin); 
    delay();
    delay();
    winput::send(Vk::Backspace);
    Mouse::set_position(90, 850);
    winput::send(Button::Left);
    //This command clears any data in the run box so we can cleanly put our CMD command in
    winput::send(Vk::Backspace);
    //This command enter cmd in the run box this will open the command prompt on windows devices
    winput::send_str("cmd");
    winput::send(Vk::Enter);
    delay();
    delay();
    //these commands will get the user name the directories they have access to.
    winput::send_str("whoami");
    winput::send(Vk::Enter);
    winput::send_str("dir");
    winput::send(Vk::Enter);
    delay();
    //These commands copy all the data that is on the screen we will use this later
    winput::press(Vk::Control); // press the shift key
    winput::send(Vk::A); // press then release the A key
    winput::release(Vk::Control); // release the shift key
    delay();
    winput::press(Vk::Control); // press the shift key
    winput::send(Vk::C); // press then release the A key
    winput::release(Vk::Control); // release the shift key
    delay();
    // This command makes a directory named test and makes two text files in the directory
    winput::send_str(" mkdir test");
    winput::send(Vk::Enter);
    winput::send_str("cd test");
    winput::send(Vk::Enter);
    winput::send_str("dir");
    winput::send(Vk::Enter);
    winput::send_str("copy con test.txt");
    winput::send(Vk::Enter);
    delay();
    //winput::send_str("this could be malicious code");
    //
    winput::press(Vk::Control); // press the control key
    winput::press(Vk::V); // press then release the V key
    winput::release(Vk::V);
    winput::release(Vk::Control); // release the Control key
    delay();
    winput::press(Vk::Control); // press the Control key
    winput::send(Vk::Z); // press then release the Z key
    winput::release(Vk::Control);
    winput::send(Vk::Enter);
    winput::send_str("copy con test2.txt");
    winput::send(Vk::Enter);
    winput::send_str("this also could be malicious code in test2.txt");
    winput::send(Vk::Enter);
    winput::press(Vk::Control); // press the Control key
    winput::send(Vk::Z); // press then release the Z key
    winput::release(Vk::Control);
    winput::send(Vk::Enter);
    winput::send_str("test.txt");
    winput::send(Vk::Enter);
    delay();
    winput::send_str("test2.txt");
    winput::send(Vk::Enter);
    winput::send_str("dir");
    winput::send(Vk::Enter);
    winput::send_str("cd ..");
    winput::send(Vk::Enter);
    
}

