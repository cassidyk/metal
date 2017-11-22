use io::PortPair;

const PIC_MASTER_COMMAND: u16   = 0x0020;
const PIC_MASTER_DATA:  u16     = 0x0021;
const PIC_SLAVE_COMMAND: u16    = 0x00A0;
const PIC_SLAVE_DATA: u16       = 0x00A1;

const ICW1_ICW4: u8 = 0x01;
const ICW1_INIT: u8 = 0x10;

const ICW1_8086: u8 = 0x01;


/// Remap the PIC!
pub unsafe fn remap(offset1: u8, offset2: u8) {
    // open PIC ports
    let mut master = PortPair::new(PIC_MASTER_COMMAND, PIC_MASTER_DATA);
    let mut slave = PortPair::new(PIC_SLAVE_COMMAND, PIC_SLAVE_DATA);

    // save masks
    let mask1 = master.read_data();
    let mask2 = slave.read_data();

    // start the init sequence
    master.write_cmd(ICW1_INIT + ICW1_ICW4);

    slave.write_cmd(ICW1_INIT + ICW1_ICW4);

    master.write_data(offset1);

    slave.write_data(offset2);

    master.write_data(4);
    slave.write_data(2);

    master.write_data(ICW1_8086);
    slave.write_data(ICW1_8086);

    // restore masks
    master.write_data(mask1);
    slave.write_data(mask2);
}