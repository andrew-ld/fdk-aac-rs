static SOURCES: &'static [&'static str] = &[
    "aac/libAACdec/src/FDK_delay.cpp",
    "aac/libAACdec/src/aac_ram.cpp",
    "aac/libAACdec/src/aac_rom.cpp",
    "aac/libAACdec/src/aacdec_drc.cpp",
    "aac/libAACdec/src/aacdec_hcr.cpp",
    "aac/libAACdec/src/aacdec_hcr_bit.cpp",
    "aac/libAACdec/src/aacdec_hcrs.cpp",
    "aac/libAACdec/src/aacdec_pns.cpp",
    "aac/libAACdec/src/aacdec_tns.cpp",
    "aac/libAACdec/src/aacdecoder.cpp",
    "aac/libAACdec/src/aacdecoder_lib.cpp",
    "aac/libAACdec/src/block.cpp",
    "aac/libAACdec/src/channel.cpp",
    "aac/libAACdec/src/channelinfo.cpp",
    "aac/libAACdec/src/conceal.cpp",
    "aac/libAACdec/src/ldfiltbank.cpp",
    "aac/libAACdec/src/pulsedata.cpp",
    "aac/libAACdec/src/rvlc.cpp",
    "aac/libAACdec/src/rvlcbit.cpp",
    "aac/libAACdec/src/rvlcconceal.cpp",
    "aac/libAACdec/src/stereo.cpp",
    "aac/libAACdec/src/usacdec_ace_d4t64.cpp",
    "aac/libAACdec/src/usacdec_ace_ltp.cpp",
    "aac/libAACdec/src/usacdec_acelp.cpp",
    "aac/libAACdec/src/usacdec_fac.cpp",
    "aac/libAACdec/src/usacdec_lpc.cpp",
    "aac/libAACdec/src/usacdec_lpd.cpp",
    "aac/libAACdec/src/usacdec_rom.cpp",
    "aac/libAACenc/src/aacEnc_ram.cpp",
    "aac/libAACenc/src/aacEnc_rom.cpp",
    "aac/libAACenc/src/aacenc.cpp",
    "aac/libAACenc/src/aacenc_lib.cpp",
    "aac/libAACenc/src/aacenc_pns.cpp",
    "aac/libAACenc/src/aacenc_tns.cpp",
    "aac/libAACenc/src/adj_thr.cpp",
    "aac/libAACenc/src/band_nrg.cpp",
    "aac/libAACenc/src/bandwidth.cpp",
    "aac/libAACenc/src/bit_cnt.cpp",
    "aac/libAACenc/src/bitenc.cpp",
    "aac/libAACenc/src/block_switch.cpp",
    "aac/libAACenc/src/channel_map.cpp",
    "aac/libAACenc/src/chaosmeasure.cpp",
    "aac/libAACenc/src/dyn_bits.cpp",
    "aac/libAACenc/src/grp_data.cpp",
    "aac/libAACenc/src/intensity.cpp",
    "aac/libAACenc/src/line_pe.cpp",
    "aac/libAACenc/src/metadata_compressor.cpp",
    "aac/libAACenc/src/metadata_main.cpp",
    "aac/libAACenc/src/mps_main.cpp",
    "aac/libAACenc/src/ms_stereo.cpp",
    "aac/libAACenc/src/noisedet.cpp",
    "aac/libAACenc/src/pnsparam.cpp",
    "aac/libAACenc/src/pre_echo_control.cpp",
    "aac/libAACenc/src/psy_configuration.cpp",
    "aac/libAACenc/src/psy_main.cpp",
    "aac/libAACenc/src/qc_main.cpp",
    "aac/libAACenc/src/quantize.cpp",
    "aac/libAACenc/src/sf_estim.cpp",
    "aac/libAACenc/src/spreading.cpp",
    "aac/libAACenc/src/tonality.cpp",
    "aac/libAACenc/src/transform.cpp",
    "aac/libArithCoding/src/ac_arith_coder.cpp",
    "aac/libDRCdec/src/FDK_drcDecLib.cpp",
    "aac/libDRCdec/src/drcDec_gainDecoder.cpp",
    "aac/libDRCdec/src/drcDec_reader.cpp",
    "aac/libDRCdec/src/drcDec_rom.cpp",
    "aac/libDRCdec/src/drcDec_selectionProcess.cpp",
    "aac/libDRCdec/src/drcDec_tools.cpp",
    "aac/libDRCdec/src/drcGainDec_init.cpp",
    "aac/libDRCdec/src/drcGainDec_preprocess.cpp",
    "aac/libDRCdec/src/drcGainDec_process.cpp",
    "aac/libFDK/src/FDK_bitbuffer.cpp",
    "aac/libFDK/src/FDK_core.cpp",
    "aac/libFDK/src/FDK_crc.cpp",
    "aac/libFDK/src/FDK_decorrelate.cpp",
    "aac/libFDK/src/FDK_hybrid.cpp",
    "aac/libFDK/src/FDK_lpc.cpp",
    "aac/libFDK/src/FDK_matrixCalloc.cpp",
    "aac/libFDK/src/FDK_qmf_domain.cpp",
    "aac/libFDK/src/FDK_tools_rom.cpp",
    "aac/libFDK/src/FDK_trigFcts.cpp",
    "aac/libFDK/src/autocorr2nd.cpp",
    "aac/libFDK/src/dct.cpp",
    "aac/libFDK/src/fft.cpp",
    "aac/libFDK/src/fft_rad2.cpp",
    "aac/libFDK/src/fixpoint_math.cpp",
    "aac/libFDK/src/huff_nodes.cpp",
    "aac/libFDK/src/mdct.cpp",
    "aac/libFDK/src/nlc_dec.cpp",
    "aac/libFDK/src/qmf.cpp",
    "aac/libFDK/src/scale.cpp",
    "aac/libMpegTPDec/src/tpdec_adif.cpp",
    "aac/libMpegTPDec/src/tpdec_adts.cpp",
    "aac/libMpegTPDec/src/tpdec_asc.cpp",
    "aac/libMpegTPDec/src/tpdec_drm.cpp",
    "aac/libMpegTPDec/src/tpdec_latm.cpp",
    "aac/libMpegTPDec/src/tpdec_lib.cpp",
    "aac/libMpegTPEnc/src/tpenc_adif.cpp",
    "aac/libMpegTPEnc/src/tpenc_adts.cpp",
    "aac/libMpegTPEnc/src/tpenc_asc.cpp",
    "aac/libMpegTPEnc/src/tpenc_latm.cpp",
    "aac/libMpegTPEnc/src/tpenc_lib.cpp",
    "aac/libPCMutils/src/limiter.cpp",
    "aac/libPCMutils/src/pcm_utils.cpp",
    "aac/libPCMutils/src/pcmdmx_lib.cpp",
    "aac/libSACdec/src/sac_bitdec.cpp",
    "aac/libSACdec/src/sac_calcM1andM2.cpp",
    "aac/libSACdec/src/sac_dec.cpp",
    "aac/libSACdec/src/sac_dec_conceal.cpp",
    "aac/libSACdec/src/sac_dec_lib.cpp",
    "aac/libSACdec/src/sac_process.cpp",
    "aac/libSACdec/src/sac_qmf.cpp",
    "aac/libSACdec/src/sac_reshapeBBEnv.cpp",
    "aac/libSACdec/src/sac_rom.cpp",
    "aac/libSACdec/src/sac_smoothing.cpp",
    "aac/libSACdec/src/sac_stp.cpp",
    "aac/libSACdec/src/sac_tsd.cpp",
    "aac/libSACenc/src/sacenc_bitstream.cpp",
    "aac/libSACenc/src/sacenc_delay.cpp",
    "aac/libSACenc/src/sacenc_dmx_tdom_enh.cpp",
    "aac/libSACenc/src/sacenc_filter.cpp",
    "aac/libSACenc/src/sacenc_framewindowing.cpp",
    "aac/libSACenc/src/sacenc_huff_tab.cpp",
    "aac/libSACenc/src/sacenc_lib.cpp",
    "aac/libSACenc/src/sacenc_nlc_enc.cpp",
    "aac/libSACenc/src/sacenc_onsetdetect.cpp",
    "aac/libSACenc/src/sacenc_paramextract.cpp",
    "aac/libSACenc/src/sacenc_staticgain.cpp",
    "aac/libSACenc/src/sacenc_tree.cpp",
    "aac/libSACenc/src/sacenc_vectorfunctions.cpp",
    "aac/libSBRdec/src/HFgen_preFlat.cpp",
    "aac/libSBRdec/src/env_calc.cpp",
    "aac/libSBRdec/src/env_dec.cpp",
    "aac/libSBRdec/src/env_extr.cpp",
    "aac/libSBRdec/src/hbe.cpp",
    "aac/libSBRdec/src/huff_dec.cpp",
    "aac/libSBRdec/src/psbitdec.cpp",
    "aac/libSBRdec/src/psdec.cpp",
    "aac/libSBRdec/src/psdec_drm.cpp",
    "aac/libSBRdec/src/psdecrom_drm.cpp",
    "aac/libSBRdec/src/pvc_dec.cpp",
    "aac/libSBRdec/src/sbr_deb.cpp",
    "aac/libSBRdec/src/sbr_dec.cpp",
    "aac/libSBRdec/src/sbr_ram.cpp",
    "aac/libSBRdec/src/sbr_rom.cpp",
    "aac/libSBRdec/src/sbrdec_drc.cpp",
    "aac/libSBRdec/src/sbrdec_freq_sca.cpp",
    "aac/libSBRdec/src/sbrdecoder.cpp",
    "aac/libSBRdec/src/lpp_tran.cpp",
    "aac/libSBRenc/src/bit_sbr.cpp",
    "aac/libSBRenc/src/code_env.cpp",
    "aac/libSBRenc/src/env_bit.cpp",
    "aac/libSBRenc/src/env_est.cpp",
    "aac/libSBRenc/src/fram_gen.cpp",
    "aac/libSBRenc/src/invf_est.cpp",
    "aac/libSBRenc/src/mh_det.cpp",
    "aac/libSBRenc/src/nf_est.cpp",
    "aac/libSBRenc/src/ps_bitenc.cpp",
    "aac/libSBRenc/src/ps_encode.cpp",
    "aac/libSBRenc/src/ps_main.cpp",
    "aac/libSBRenc/src/resampler.cpp",
    "aac/libSBRenc/src/sbr_encoder.cpp",
    "aac/libSBRenc/src/sbr_misc.cpp",
    "aac/libSBRenc/src/sbrenc_freq_sca.cpp",
    "aac/libSBRenc/src/sbrenc_ram.cpp",
    "aac/libSBRenc/src/sbrenc_rom.cpp",
    "aac/libSBRenc/src/ton_corr.cpp",
    "aac/libSBRenc/src/tran_det.cpp",
    "aac/libSYS/src/genericStds.cpp",
    "aac/libSYS/src/syslib_channelMapDescr.cpp",    
];

static INCLUDE_DIRS: &'static [&'static str] = &[
    "aac/libAACenc/include",
    "aac/libAACdec/include",
    "aac/libArithCoding/include",
    "aac/libDRCdec/include",
    "aac/libFDK/include",
    "aac/libMpegTPDec/include",
    "aac/libMpegTPEnc/include",
    "aac/libPCMutils/include",
    "aac/libSACdec/include",
    "aac/libSACenc/include",
    "aac/libSBRdec/include",
    "aac/libSBRenc/include",
    "aac/libSYS/include",
];

fn main() {
    for src in SOURCES {
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed={}", src);
    }

    let mut cc = cc::Build::new();

    cc.warnings(false);
    cc.files(SOURCES);

    if let Ok(ndk_headers_path) = std::env::var("NDK_HEADERS_PATH") {
        cc.include(ndk_headers_path);
    }

    for include in INCLUDE_DIRS {
        cc.include(include);
    }

    cc.compile("libfdk-aac.a");

    println!("cargo:rustc-link-lib=fdk-aac");

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    }
}
