use c64_assembler::{
    instruction::operation::Operation,
    validator::{AssemblerResult, Error, Validator},
};
use c64_assembler_macro::application;

#[test]
fn address_names_not_exist() -> AssemblerResult<()> {
    let application = application!(module!(instruction!(
        jmp unknown_label
    )))?;
    let result = application.validate();
    if let Err(Error::AddressNameUnknown(label)) = result {
        assert_eq!(&label, "unknown_label");
    }
    Ok(())
}

#[test]
fn address_names_exist() -> AssemblerResult<()> {
    let application = application!(module!(instruction!(
            jmp known_label
        known_label:
    )))?;

    let result = application.validate();
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn address_names_unique() -> AssemblerResult<()> {
    let application = application!(module!(instruction!(
        unique_label_a:
        unique_label_b:
    )))?;

    let result = application.validate();
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn address_names_unique_modules() -> AssemblerResult<()> {
    let application = application!(module!(instruction!(
            unique_label_a:
        ))
        module!(instructions!(
            unique_label_b:
        ))
    )?;

    let result = application.validate();
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn address_names_not_unique_one_module() -> AssemblerResult<()> {
    let application = application!(module!(instructions!(
        not_unique_label:
        not_unique_label:
    )))?;

    let result = application.validate();
    match result {
        Err(Error::AddressNameNotUnique(label)) => assert_eq!(&label, "not_unique_label"),
        _ => {
            unreachable!()
        }
    }
    Ok(())
}

#[test]
fn address_names_not_unique_one_module_function() -> AssemblerResult<()> {
    let application = application!(module!(instructions!(
    not_unique_label:
    )
    function!(
        instructions!(
    not_unique_label:
    ))))?;

    let result = application.validate();
    match result {
        Err(Error::AddressNameNotUnique(label)) => assert_eq!(&label, "not_unique_label"),
        _ => {
            unreachable!()
        }
    }
    Ok(())
}

#[test]
fn address_names_not_unique_modules() -> AssemblerResult<()> {
    let application = application!(module!(instructions!(
            not_unique_label:
        ))
        module!(instructions!(
            not_unique_label:
        ))
    )?;

    let result = application.validate();
    match result {
        Err(Error::AddressNameNotUnique(label)) => assert_eq!(&label, "not_unique_label"),
        _ => {
            unreachable!()
        }
    }
    Ok(())
}

/// It is a common pattern to put a label at the end of an program for scratch space.
#[test]
fn label_at_end_of_instructions() -> AssemblerResult<()> {
    let application = application!(module!(instructions!(
    label_a:
    )))?;
    assert_eq!(1, application.modules.len());
    assert_eq!(1, application.modules.get(0).unwrap().instructions.instructions.len());
    if let Operation::Label(label) = &application
        .modules
        .get(0)
        .unwrap()
        .instructions
        .instructions
        .get(0)
        .unwrap()
        .operation
    {
        assert_eq!("label_a", label);
    } else {
        unreachable!("no label instruction")
    }
    Ok(())
}
