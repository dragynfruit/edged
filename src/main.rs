use std::{fs::File, io::{Read, Write}};
use flatbuffers::Follow;
use flexbuffers::Buffer;

use edged::flatbuffers::{executable::platforms::darwinn::{Executable, MultiExecutable}, tflite_model::tflite::{Model, ModelOffset, SubGraph}};

fn main() {
    unsafe {
        let mut buf = Vec::new();
        let mut f = File::open(std::env::args().next().unwrap()).unwrap();
        f.read_to_end(&mut buf).unwrap();

        let (prefix, shorts, suffix) = buf.align_to::<u16>();
        dbg!(prefix, suffix);
        //let model = Model::follow(&buf, ModelOffset);
        let model = flatbuffers::root_unchecked::<Model>(&buf[8..]);
        dbg!(model);
        //let model = edged::flatbuffers::tflite_model::tflite::root_as_model(
        //    &buf,
        //).expect("failed spectacularly");

        dbg!(model.description().unwrap(), model.operator_codes().unwrap().iter().next().unwrap(), model.subgraphs().unwrap().iter().next().map(|x| (x.name(), x.tensors(), x.inputs(), x.outputs())).unwrap());

        //dbg!(model.buffers().unwrap().len());
        //dbg!(
        //    model.buffers().unwrap().get(0).data().unwrap().len(),
        //    model.buffers().unwrap().get(1).data().unwrap().len(),
        //    model.buffers().unwrap().get(1).offset(),
        //);
        let bufs = model.subgraphs().expect("failed to get subgraphs").get(0).operators().expect("failed to get operators").into_iter().map(|x| x.custom_options().map(|y| y.into_iter().collect::<Vec<u8>>()).unwrap_or_else(|| Vec::new()));

        println!("{}", bufs.len());

        let mut f = File::create("test-edge.bin").unwrap();

        //f.write_all(model.buffers().unwrap().get(1).data().unwrap().bytes()).unwrap();
        //f.write_all(bufs.clone().into_iter().next().unwrap().as_slice());
        {
            let buf = bufs.clone().into_iter().next().unwrap();
            for buf in bufs.clone().into_iter() {
                if buf.len() > 0 {
            let data = flexbuffers::Reader::get_root(buf.as_slice()).unwrap().as_map();

            //let data = flatbuffers::root::<flatbuffers::Table>(buf).unwrap();
            dbg!(data.iter_keys().map(|x| x.clone().to_string()).collect::<Vec<String>>());
            dbg!(
                data.index("1").unwrap(),
                data.index("4").unwrap(),
                data.index("5").unwrap(),
                data.index("6").unwrap(),
                data.index("7").unwrap(),
            );
            dbg!(data.index("1").unwrap().to_string());
            let from = 0;
            dbg!(data.index("1").unwrap().buffer().len());
            let to = data.index("4").unwrap().length();
            let dataa = data.index("4").unwrap().buffer().slice(8..to+8).unwrap();
            println!("ser len: {}", dataa.len());
            //f.write_all(data.index("4").unwrap().buffer().to_vec().as_slice()).unwrap();
            //f.write_all(&data.index("4").unwrap().buffer().to_vec().as_slice()).unwrap();
            dbg!(data.index("6").unwrap().as_vector().iter().map(|x| x.to_string()).collect::<Vec<String>>());
            dbg!(data.index("7").unwrap().as_vector().iter().map(|x| x.to_string()).collect::<Vec<String>>());
        //f.write_all(data.index("18").unwrap().as_blob().0).unwrap();
            //if let Ok(pkg) = flatbuffers::root::<edged::flatbuffers::executable::platforms::darwinn::Package>(
            //let dataa = data.clone().index("4").unwrap();
            //let dataa = dataa.as_.collect::<Vec<u8>>();
            //let pkgg = flexbuffers::Reader::get_root(dataa.as_slice()).unwrap().as_map();
            //dbg!(pkgg.iter_keys().collect::<Vec<_>>());
            let pkg = edged::flatbuffers::executable::platforms::darwinn::root_as_package(
            //&data.index("4").unwrap().buffer().to_vec().as_slice()[9..]
            //include_bytes!("../test-edge.bin")
            dataa
        ).unwrap();
            {
               // for exe in exe.serialized_executables() {
                 //   dbg!(exe.len());
                //dbg!(exe.version(), exe.name(), exe.chip(), exe.input_layers().unwrap().len());
             //   }

        dbg!(pkg.signature(), pkg.model_identifier(), pkg.min_runtime_version(), pkg.multi_chip_package().map_or(0, |x| x.bytes().len()), pkg.serialized_multi_executable().map_or(0, |x| x.bytes().len()));

        //f.write_all(pkg.serialized_multi_executable().unwrap().bytes()).unwrap();

        let execs = flatbuffers::root_unchecked::<MultiExecutable>(pkg.serialized_multi_executable().unwrap().bytes());

        for exec in execs.serialized_executables().unwrap().iter() {
            let exe = flatbuffers::root_unchecked::<Executable>(exec.as_bytes());
            if exe.name() != Some("Unknown") {
                f.write_all(exe.instruction_bitstreams().unwrap().get(0).bitstream().unwrap().bytes()).unwrap();
            }
            dbg!(exe.name(), exe.chip(), exe.scratch_size_bytes(), exe.instruction_bitstreams().unwrap().len());
        }
            }
            }
        }
        f.flush().unwrap();
        drop(f);

        for buf in bufs {
            println!("{}", buf.len());
        //if let Ok(pkg) = edged::flatbuffers::executable::platforms::darwinn::size_prefixed_root_as_package(
        //    &buf.clone()
        //) {

        //dbg!(buf.len(), pkg.signature(), pkg.model_identifier(), pkg.min_runtime_version(), pkg.multi_chip_package(), pkg.serialized_multi_executable());
        //}
        }
        }
    }
}
