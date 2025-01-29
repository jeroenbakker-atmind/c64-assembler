use c64_assembler::validator::{AssemblerResult, Error, Validator};
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
    let application = application!(module!(instruction!(
        not_unique_label:
        not_unique_label:
    )))?;

    let result = application.validate();
    if let Err(Error::AddressNameNotUnique(label)) = result {
        assert_eq!(&label, "not_unique_label");
    }
    Ok(())
}

#[test]
fn address_names_not_unique_one_module_function() -> AssemblerResult<()> {
    let application = application!(module!(instruction!(
    not_unique_label:
    )
    function!(
        instructions!(
    not_unique_label:
    ))))?;

    let result = application.validate();
    if let Err(Error::AddressNameNotUnique(label)) = result {
        assert_eq!(&label, "not_unique_label");
    }
    Ok(())
}

#[test]
fn address_names_not_unique_modules() -> AssemblerResult<()> {
    let application = application!(module!(instruction!(
            not_unique_label:
        ))
        module!(instructions!(
            not_unique_label:
        ))
    )?;

    let result = application.validate();
    if let Err(Error::AddressNameNotUnique(label)) = result {
        assert_eq!(&label, "not_unique_label");
    }
    Ok(())
}
