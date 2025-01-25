use std::collections::HashMap;

use crate::memory::{
    define::{Define, Value},
    label::AddressReference,
    Address,
};

use super::{finalize::finalize, module::Module};

#[derive(Clone)]
pub struct Application {
    pub name: String,
    pub entry_point: Address,
    pub modules: Vec<Module>,
    pub defines: Vec<Define>,
    pub address_lookup: HashMap<String, Address>,
}

#[derive(Clone)]
pub struct ApplicationBuilder {
    application: Application,
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self {
            application: Application {
                name: String::default(),
                entry_point: 0x0800,
                modules: vec![],
                defines: vec![],
                address_lookup: HashMap::default(),
            },
        }
    }
}

impl ApplicationBuilder {
    /// Set the name of the application.
    ///
    /// Is used in comments when exporting to a dasm source using [crate::generator::dasm::DasmGenerator]
    ///
    /// ```
    /// use c64_assembler::builder::application::ApplicationBuilder;
    /// let application = ApplicationBuilder::default()
    ///     .name("My application")
    ///     .finalize();
    /// ```
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.application.name = name.to_string();
        self
    }

    /// Change the entry point of the application.
    ///
    /// NOTE: When not set to 0x0800 [crate::builder::instruction::InstructionBuilder::add_basic_header]
    /// cannot be used.
    ///
    /// Default entry point is set to 0x0800.
    ///
    /// ```
    /// use c64_assembler::builder::application::ApplicationBuilder;
    /// let application = ApplicationBuilder::default()
    ///     .entry_point(0x0800)
    ///     .finalize();
    /// ```
    pub fn entry_point(&mut self, entry_point: Address) -> &mut Self {
        self.application.entry_point = entry_point;
        self
    }

    /// Define a static address.
    ///
    /// When using instructions each address needs to be accessed by its name. This function
    /// allows to add a new named address. There is no difference between zeropaged and regular
    /// addresses. When building the instruction stream the actual address will decide which
    /// opcode is used.
    ///
    /// ```
    /// use c64_assembler::builder::application::ApplicationBuilder;
    /// let application = ApplicationBuilder::default()
    ///     .define_address("VIC_BORDER_COLOR", 0xD020)
    ///     .define_address("ZEROPAGE_FE", 0xFE)
    ///     .finalize();
    /// ```
    pub fn define_address(&mut self, name: &str, address: Address) -> &mut Self {
        self.application.address_lookup.insert(name.to_string(), address);
        self.application
            .defines
            .push(Define::new(name, Value::Address(address)));
        self
    }

    /// Add the address defines useful when using the VIC20.
    ///
    /// | Address Name                     | Physical Address | Description                                      |
    /// |----------------------------------|-----------------|--------------------------------------------------|
    /// | VIC20_BASE                       | 0xD000          | Base address for VIC registers                  |
    /// | VIC20_SPRITE_0_X                 | 0xD000          | X coordinate of sprite 0                        |
    /// | VIC20_SPRITE_0_Y                 | 0xD001          | Y coordinate of sprite 0                        |
    /// | VIC20_SPRITE_1_X                 | 0xD002          | X coordinate of sprite 1                        |
    /// | VIC20_SPRITE_1_Y                 | 0xD003          | Y coordinate of sprite 1                        |
    /// | VIC20_SPRITE_2_X                 | 0xD004          | X coordinate of sprite 2                        |
    /// | VIC20_SPRITE_2_Y                 | 0xD005          | Y coordinate of sprite 2                        |
    /// | VIC20_SPRITE_3_X                 | 0xD006          | X coordinate of sprite 3                        |
    /// | VIC20_SPRITE_3_Y                 | 0xD007          | Y coordinate of sprite 3                        |
    /// | VIC20_SPRITE_4_X                 | 0xD008          | X coordinate of sprite 4                        |
    /// | VIC20_SPRITE_4_Y                 | 0xD009          | Y coordinate of sprite 4                        |
    /// | VIC20_SPRITE_5_X                 | 0xD00A          | X coordinate of sprite 5                        |
    /// | VIC20_SPRITE_5_Y                 | 0xD00B          | Y coordinate of sprite 5                        |
    /// | VIC20_SPRITE_6_X                 | 0xD00C          | X coordinate of sprite 6                        |
    /// | VIC20_SPRITE_6_Y                 | 0xD00D          | Y coordinate of sprite 6                        |
    /// | VIC20_SPRITE_7_X                 | 0xD00E          | X coordinate of sprite 7                        |
    /// | VIC20_SPRITE_7_Y                 | 0xD00F          | Y coordinate of sprite 7                        |
    /// | VIC20_SPRITE_X_MSB               | 0xD010          | Most significant bit for sprite X coordinates   |
    /// | VIC20_CONTROL_1                  | 0xD011          | Screen control register 1                       |
    /// | VIC20_RASTER                     | 0xD012          | Raster line position                            |
    /// | VIC20_LIGHT_PEN_X                | 0xD013          | Light pen X position                           |
    /// | VIC20_LIGHT_PEN_Y                | 0xD014          | Light pen Y position                           |
    /// | VIC20_SPRITE_ENABLE              | 0xD015          | Enables sprites                                |
    /// | VIC20_CONTROL_2                  | 0xD016          | Screen control register 2                      |
    /// | VIC20_SPRITE_EXPAND_X            | 0xD017          | Expands sprites horizontally                   |
    /// | VIC20_MEMORY_SETUP               | 0xD018          | VIC memory setup                               |
    /// | VIC20_IRQ_STATUS                 | 0xD019          | Interrupt request status                       |
    /// | VIC20_IRQ_ENABLE                 | 0xD01A          | Interrupt request enable                       |
    /// | VIC20_SPRITE_PRIORITY            | 0xD01B          | Sprite priority over background                |
    /// | VIC20_SPRITE_MULTICOLOR          | 0xD01C          | Enables multicolor mode for sprites            |
    /// | VIC20_SPRITE_EXPAND_Y            | 0xD01D          | Expands sprites vertically                     |
    /// | VIC20_SPRITE_COLLISION           | 0xD01E          | Sprite-to-sprite collision detection           |
    /// | VIC20_SPRITE_BG_COLLISION        | 0xD01F          | Sprite-to-background collision detection       |
    /// | VIC20_BORDER_COLOR               | 0xD020          | Border color                                   |
    /// | VIC20_BACKGROUND_COLOR           | 0xD021          | Background color                               |
    /// | VIC20_BACKGROUND_COLOR_0         | 0xD021          | Background color 0                             |
    /// | VIC20_BACKGROUND_COLOR_1         | 0xD022          | Background color 1                             |
    /// | VIC20_BACKGROUND_COLOR_2         | 0xD023          | Background color 2                             |
    /// | VIC20_BACKGROUND_COLOR_3         | 0xD024          | Background color 3                             |
    /// | VIC20_SPRITE_MULTICOLOR_0        | 0xD025          | Multicolor mode color 0 for sprites           |
    /// | VIC20_SPRITE_MULTICOLOR_1        | 0xD026          | Multicolor mode color 1 for sprites           |
    /// | VIC20_SPRITE_0_COLOR             | 0xD027          | Color of sprite 0                              |
    /// | VIC20_SPRITE_1_COLOR             | 0xD028          | Color of sprite 1                              |
    /// | VIC20_SPRITE_2_COLOR             | 0xD029          | Color of sprite 2                              |
    /// | VIC20_SPRITE_3_COLOR             | 0xD02A          | Color of sprite 3                              |
    /// | VIC20_SPRITE_4_COLOR             | 0xD02B          | Color of sprite 4                              |
    /// | VIC20_SPRITE_5_COLOR             | 0xD02C          | Color of sprite 5                              |
    /// | VIC20_SPRITE_6_COLOR             | 0xD02D          | Color of sprite 6                              |
    /// | VIC20_SPRITE_7_COLOR             | 0xD02E          | Color of sprite 7                              |
    ///
    /// ```
    /// use c64_assembler::builder::application::ApplicationBuilder;
    /// let application = ApplicationBuilder::default()
    ///     .include_vic20_defines()
    ///     .finalize();
    /// ```
    pub fn include_vic20_defines(&mut self) -> &mut Self {
        self.define_address("VIC20_BASE", 0xD000)
            .define_address("VIC20_SPRITE_0_X", 0xD000)
            .define_address("VIC20_SPRITE_0_Y", 0xD001)
            .define_address("VIC20_SPRITE_1_X", 0xD002)
            .define_address("VIC20_SPRITE_1_Y", 0xD003)
            .define_address("VIC20_SPRITE_2_X", 0xD004)
            .define_address("VIC20_SPRITE_2_Y", 0xD005)
            .define_address("VIC20_SPRITE_3_X", 0xD006)
            .define_address("VIC20_SPRITE_3_Y", 0xD007)
            .define_address("VIC20_SPRITE_4_X", 0xD008)
            .define_address("VIC20_SPRITE_4_Y", 0xD009)
            .define_address("VIC20_SPRITE_5_X", 0xD00A)
            .define_address("VIC20_SPRITE_5_Y", 0xD00B)
            .define_address("VIC20_SPRITE_6_X", 0xD00C)
            .define_address("VIC20_SPRITE_6_Y", 0xD00D)
            .define_address("VIC20_SPRITE_7_X", 0xD00E)
            .define_address("VIC20_SPRITE_7_Y", 0xD00F)
            .define_address("VIC20_SPRITE_X_MSB", 0xD010)
            .define_address("VIC20_CONTROL_1", 0xD011)
            .define_address("VIC20_RASTER", 0xD012)
            .define_address("VIC20_LIGHT_PEN_X", 0xD013)
            .define_address("VIC20_LIGHT_PEN_Y", 0xD014)
            .define_address("VIC20_SPRITE_ENABLE", 0xD015)
            .define_address("VIC20_CONTROL_2", 0xD016)
            .define_address("VIC20_SPRITE_EXPAND_X", 0xD017)
            .define_address("VIC20_MEMORY_SETUP", 0xD018)
            .define_address("VIC20_IRQ_STATUS", 0xD019)
            .define_address("VIC20_IRQ_ENABLE", 0xD01A)
            .define_address("VIC20_SPRITE_PRIORITY", 0xD01B)
            .define_address("VIC20_SPRITE_MULTICOLOR", 0xD01C)
            .define_address("VIC20_SPRITE_EXPAND_Y", 0xD01D)
            .define_address("VIC20_SPRITE_COLLISION", 0xD01E)
            .define_address("VIC20_SPRITE_BG_COLLISION", 0xD01F)
            .define_address("VIC20_BORDER_COLOR", 0xD020)
            .define_address("VIC20_BACKGROUND_COLOR", 0xD021)
            .define_address("VIC20_BACKGROUND_COLOR_0", 0xD021)
            .define_address("VIC20_BACKGROUND_COLOR_1", 0xD022)
            .define_address("VIC20_BACKGROUND_COLOR_2", 0xD023)
            .define_address("VIC20_BACKGROUND_COLOR_3", 0xD024)
            .define_address("VIC20_SPRITE_MULTICOLOR_0", 0xD025)
            .define_address("VIC20_SPRITE_MULTICOLOR_1", 0xD026)
            .define_address("VIC20_SPRITE_0_COLOR", 0xD027)
            .define_address("VIC20_SPRITE_1_COLOR", 0xD028)
            .define_address("VIC20_SPRITE_2_COLOR", 0xD029)
            .define_address("VIC20_SPRITE_3_COLOR", 0xD02A)
            .define_address("VIC20_SPRITE_4_COLOR", 0xD02B)
            .define_address("VIC20_SPRITE_5_COLOR", 0xD02C)
            .define_address("VIC20_SPRITE_6_COLOR", 0xD02D)
            .define_address("VIC20_SPRITE_7_COLOR", 0xD02E)
    }

    /// Add the address defines useful when using the SID.
    ///
    /// | Address Name                  | Physical Address | Description                                    |
    /// |-------------------------------|-----------------|-------------------------------------------------|
    /// | SID_BASE                      | 0xD400          | Base address for SID registers                  |
    /// | SID_VOICE_1_FREQ_LO           | 0xD400          | Voice 1 frequency (low byte)                    |
    /// | SID_VOICE_1_FREQ_HI           | 0xD401          | Voice 1 frequency (high byte)                   |
    /// | SID_VOICE_1_PW_LO             | 0xD402          | Voice 1 pulse width (low byte)                  |
    /// | SID_VOICE_1_PW_HI             | 0xD403          | Voice 1 pulse width (high byte)                 |
    /// | SID_VOICE_1_CTRL              | 0xD404          | Voice 1 control register                        |
    /// | SID_VOICE_1_AD                | 0xD405          | Voice 1 attack/decay settings                   |
    /// | SID_VOICE_1_SR                | 0xD406          | Voice 1 sustain/release settings                |
    /// | SID_VOICE_2_FREQ_LO           | 0xD407          | Voice 2 frequency (low byte)                    |
    /// | SID_VOICE_2_FREQ_HI           | 0xD408          | Voice 2 frequency (high byte)                   |
    /// | SID_VOICE_2_PW_LO             | 0xD409          | Voice 2 pulse width (low byte)                  |
    /// | SID_VOICE_2_PW_HI             | 0xD40A          | Voice 2 pulse width (high byte)                 |
    /// | SID_VOICE_2_CTRL              | 0xD40B          | Voice 2 control register                        |
    /// | SID_VOICE_2_AD                | 0xD40C          | Voice 2 attack/decay settings                   |
    /// | SID_VOICE_2_SR                | 0xD40D          | Voice 2 sustain/release settings                |
    /// | SID_VOICE_3_FREQ_LO           | 0xD40E          | Voice 3 frequency (low byte)                    |
    /// | SID_VOICE_3_FREQ_HI           | 0xD40F          | Voice 3 frequency (high byte)                   |
    /// | SID_VOICE_3_PW_LO             | 0xD410          | Voice 3 pulse width (low byte)                  |
    /// | SID_VOICE_3_PW_HI             | 0xD411          | Voice 3 pulse width (high byte)                 |
    /// | SID_VOICE_3_CTRL              | 0xD412          | Voice 3 control register                        |
    /// | SID_VOICE_3_AD                | 0xD413          | Voice 3 attack/decay settings                   |
    /// | SID_VOICE_3_SR                | 0xD414          | Voice 3 sustain/release settings                |
    /// | SID_FILTER_CUTOFF_LO          | 0xD415          | Filter cutoff frequency (low byte)              |
    /// | SID_FILTER_CUTOFF_HI          | 0xD416          | Filter cutoff frequency (high byte)             |
    /// | SID_FILTER_CTRL               | 0xD417          | Filter control register                         |
    /// | SID_VOLUME_FC                 | 0xD418          | Volume and filter control                       |
    /// | SID_POT_X                     | 0xD419          | Paddle X position (read)                        |
    /// | SID_POT_Y                     | 0xD41A          | Paddle Y position (read)                        |
    /// | SID_OSC3_RANDOM               | 0xD41B          | Oscillator 3 output/random number generator     |
    /// | SID_ENV3                      | 0xD41C          | Envelope generator for voice 3 (read)           |
    ///
    /// ```
    /// use c64_assembler::builder::application::ApplicationBuilder;
    /// let application = ApplicationBuilder::default()
    ///     .include_sid_defines()
    ///     .finalize();
    /// ```
    pub fn include_sid_defines(&mut self) -> &mut Self {
        self.define_address("SID_BASE", 0xD400)
            .define_address("SID_VOICE_1_FREQ_LO", 0xD400)
            .define_address("SID_VOICE_1_FREQ_HI", 0xD401)
            .define_address("SID_VOICE_1_PW_LO", 0xD402)
            .define_address("SID_VOICE_1_PW_HI", 0xD403)
            .define_address("SID_VOICE_1_CTRL", 0xD404)
            .define_address("SID_VOICE_1_AD", 0xD405)
            .define_address("SID_VOICE_1_SR", 0xD406)
            .define_address("SID_VOICE_2_FREQ_LO", 0xD407)
            .define_address("SID_VOICE_2_FREQ_HI", 0xD408)
            .define_address("SID_VOICE_2_PW_LO", 0xD409)
            .define_address("SID_VOICE_2_PW_HI", 0xD40A)
            .define_address("SID_VOICE_2_CTRL", 0xD40B)
            .define_address("SID_VOICE_2_AD", 0xD40C)
            .define_address("SID_VOICE_2_SR", 0xD40D)
            .define_address("SID_VOICE_3_FREQ_LO", 0xD40E)
            .define_address("SID_VOICE_3_FREQ_HI", 0xD40F)
            .define_address("SID_VOICE_3_PW_LO", 0xD410)
            .define_address("SID_VOICE_3_PW_HI", 0xD411)
            .define_address("SID_VOICE_3_CTRL", 0xD412)
            .define_address("SID_VOICE_3_AD", 0xD413)
            .define_address("SID_VOICE_3_SR", 0xD414)
            .define_address("SID_FILTER_CUTOFF_LO", 0xD415)
            .define_address("SID_FILTER_CUTOFF_HI", 0xD416)
            .define_address("SID_FILTER_CTRL", 0xD417)
            .define_address("SID_VOLUME_FC", 0xD418)
            .define_address("SID_POT_X", 0xD419)
            .define_address("SID_POT_Y", 0xD41A)
            .define_address("SID_OSC3_RANDOM", 0xD41B)
            .define_address("SID_ENV3", 0xD41C)
    }

    pub fn module(&mut self, module: Module) -> &mut Self {
        self.application.modules.push(module);
        self
    }

    /// Build the application
    pub fn finalize(&mut self) -> Application {
        finalize(&mut self.application);
        self.application.clone()
    }
}

impl Application {
    pub(crate) fn define_mut(&mut self, define_name: &String) -> &mut Define {
        self.defines
            .iter_mut()
            .find(|define| &define.name == define_name)
            .unwrap()
    }

    pub fn address(&self, address_reference: &AddressReference) -> Address {
        self.address_lookup.get(&address_reference.name).unwrap() + address_reference.offset
    }
}
