todo

hf only no vhf for a long while

Signal - type
    ALL signals will have identity, frequency, strength, mode, location
    Identity - ie call even if not possible
    Frequency - ie band
    Strength - changes as it propagates from transmitter -> line -> antenna -> skywave/etc -> antenna -> line -> reciever
        represents total gain (loss) of system (dB)
        attached is initial power outputted by transmitter (not public unless included by mode)
    Originating Location (not public unless included in payload)
        includes angle from source to reciever
    Mode
        Carrier - technically conveys no information but location
        CW
        Voice
    Payload
        Message
        Location if in mode

Attachable to transmission line:
Loads
    Dipole (done)
    DummyLoad
Sources - 50ohm impedance typ ie what is z_s on transmission line
    AntennaAnalyzer
        reports impedance + swr for a frequency
        attached directly to antenna or to transmission line
        far too low power to be worth propagating, simply reads the antenna and transmission line match and presents SWR and impedance
    Transmitter
        places a signal to be propagated (with a power level attached) -> travels across transmission line (losses) -> radiated by antenna (directivity) -> recieved elsewhere
    Reciever
        not a signal source, but it is placed here to attach to the line
        recieves signal propagated on antenna -> losses/gain by antenna directivity at azumith/maybe elevation -> loss by transmission line -> reciever "power" ie sensitivity
        has to deal with noise levels present in environment (ie from atmospheric noise or local environmental rfi)
            strong signals nearby if poor selectivity perhaps (ie broadcast tower 3 km away)
    Transciever
        combo of two - later on

Propagation
    at first, simple distance and angle will be used to calculate the recieved power
        free space path loss + directional gain + polarization mismatch?
        noise present as well
    recievers will have a minimum snr for mode?
    later on band simulations maybe + final propagation model if possible