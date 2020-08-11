//导入所需的库
use std::io::{Error, Read, Write};
//网络库
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;
//用于处理tcp监听到的流的函数
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    //创建buffer
    let mut buf = [0; 512];
    for _ in 0..1000 {
        //从流里读内容
        let bytes_read = stream.read(&mut buf)?;
        //如果是0 说明读完 返回ok
        if bytes_read == 0 {
            return Ok(());
        }
        
        //写回
        stream.write(&buf[..bytes_read])?;
        //打印
        let info = &buf[..bytes_read];
        //打印server收到的数据，字符被转换为int
        for i in info.iter() {
            println!("{}", i);
        }
        
            
        //println!("{}", str::from_utf8(&buf));
        //睡眠1s
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    //创建监听 绑定端口8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;//需要做错误处理
    // 创建容器存放线程
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // 获得监听得到的流
    for stream in listener.incoming() {
        //转换stream 如果有问题打印failed
        let stream = stream.expect("failed!");
        //闭包去处理
        let handle = thread::spawn(move || {
            //调用处理函数
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error));//unwrap是处理错误的
        });
        //handle加入容器
        thread_vec.push(handle);
    }
    //等待线程结束
    for handle in thread_vec {
        handle.join().unwrap();
    }
    //错误处理 返回ok
    Ok(())
}