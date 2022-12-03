use std::collections::HashSet;

pub fn sum_priorities() -> u32 {
    let mut sum = 0;
    let lines = DATA.split('\n');
    for line in lines {
        let l = line.len() / 2;
        let v = line.chars().collect::<Vec<char>>();
        let chunks = v.chunks(l);
        let mut first_compartment = HashSet::new();
        let mut second_compartment = HashSet::new();
        for (i, chunk) in chunks.into_iter().enumerate() {
            for c in chunk {
                if i == 0 {
                    first_compartment.insert(*c);
                } else {
                    second_compartment.insert(*c);
                }
            }
        }

        for c in first_compartment {
            if second_compartment.contains(&c) {
                sum = sum + get_char_number(c);
            }
        }
    }

    sum
}

pub fn three_rucksack_priority() -> u32 {
    let mut sum = 0;
    let lines = DATA.split('\n').collect::<Vec<&str>>();
    let line_chunks = lines.chunks(3);

    for line_chunk in line_chunks {
        let mut first_compartment = HashSet::new();
        let mut second_compartment = HashSet::new();

        for (i, chunk) in line_chunk.iter().enumerate() {
            for char in chunk.chars() {
                if i == 0 {
                    first_compartment.insert(char);
                } else if i == 1 {
                    second_compartment.insert(char);
                } else {
                    if first_compartment.contains(&char) && second_compartment.contains(&char) {
                        sum = sum + get_char_number(char);
                        break; // In the case there are duplicates in the third, break out to only count once
                    }
                }
            }
        }
    }

    sum
}

fn get_char_number(c: char) -> u32 {
    let upper = c.is_ascii_uppercase();
    let u: u32 = c.into();
    if upper {
        u - 38
    } else {
        u - 96
    }
}

const DATA: &str = "WwcsbsWwspmFTGVV
RHtMDHdSMnDBGMSDvnvDjtmpTpjTFggpmjmTFggTjmpP
vtCSGRMBDzHddvBHBzRhrlcZhlLzWNlqblhzcr
shhszHNHHZWqSzVNdClMjlFjBBbNTB
tQQGmnrMnJnGfmvrRRPCjlbljFBdjFCjTjnP
mRwtfGrMmJtwRDvQJQrJpMLSzVDHzhzHZqZzqSzcWVWH
WsWWgrtgsrhTQtsFcWPcRMCCTvqvMvqNNqMMHlMq
bBJrBGbzzLJznJrbSDGGJLqmlvqMqvlmLHRqRZZRNZ
bzJfDGVSzVrJGwjVGPPpQthdPsPpjdphsc
pJpCCBSWlczWWBWMHdMmMsFmpddrgF
wfVqZZGVQvzsMqmMgHjm
vDZGvPttQTVtGDQDDDGwbSCcSJSCJWTcRRSRczRJ
HLVHsVWLwbWswbpWFWrrmThfTPNnhNSDDNhDfznTnhnS
pBRcvGvvBtpGcqqQvgcphPfzfDGhzdzPDzDDhnhS
ZQRvqBptjJgZCtJqqMMMLHWwMWZWHHFFHm
PvPFPvLLLSvNFvQNWNPvrPLrZjwhMttTwtTtQZBwqjqtZqwM
HJDDbHjgppzCDCmzpgzsGbCsTMZqZllqhJBhMTtVBBhMtMth
zgGncmGGzHCnHDpDgDCGsmFLLPFjPRRWLRjdcjrcdRLd
zHnWzntnBRWTSBzRBddpFvZVcHpLFvjvLppvHP
MmmWmNGQhbCpZVLLbccvpj
QDMCGrNWfwNznBJsJzDBdg
tcRcZccZmdZJctRcjrlhNNDfrdNdSfNsNT
QHQpBVvMpRMwgBgvnHRFlhrSsgNFThgTFFflNS
vvHpVBBBGBppHvpLvHGbjmmtCqWLJJZRzZZZZb
ZBtTDZRWsTsDZVWVZDmjpbLbpSSzmLpWrbrS
MFNNFvvwFHwlhmNrCStLNtjzrb
vwffwcHwflGqGflHJfDBBZtQVBgZQJtBBsnT
pTJcmMJTspmpMZZJJZHCQQMzPBlQdWWWFzWP
LDnwrdnDnqjfqgvfDjrfFlBBPFHFSHPQCBvQSSWB
nLbjgLjdbrwVRcppsscJVRRR
mHnfggmMtpHPPBCs
PJjlQQRrJhJNPPTtBsCbCCTlpptd
rSSDhNQwShRRjhmMPmzMDfPmfLzL
HzLFBgrCthtFrrhFSCCCvBQNRVmJJJmnpnddmppddVtJ
MPZsjDWPjZsVzNTzpVdRdZ
qMfjWfwclsPsjwzqHgLFhwGFwHrFFrSC
llllmSbhNmSbNzlPmRNCcgLLchHHpTGsCTQGpT
dVjBrvBBVLJQsLpC
frZBWBDMFndStFsSwzlPlq
vmTVVtmJHwCwDllttTsrcPcMrfqPMMpjMq
LQGBRgGGRNgGgBhgzHfpjPqsMjpLcLjrPLpq
BdgzgSRGBnNHJtJlVStVmt
FbDQsFjPVHFZFSbrVjSVvMJlGBJhDcqBBllJGccJnh
RfTCTTpmppfgwCpwpLwRMnMGMlcPGqhddPcJnl
zgLPLNCCpLggzmTzTWmVrjVvrNvjjjvbVHQZZH
RBjjpwmRszBdvhLdSvpVpV
GrbfbJWmQJGWrGZZQMbSLggfCgSHhCSgShghSC
DWNDZQcrbWQrZJZGQQZPsztzBsPmBTzwcwRwjT
rlvgglvZqbrbWbWWdvdmPHBBNMNJGqjGRRnHnPBJ
cDFDcfcCDhLzsCfLDVpGPRJMPsRJMPNRnjHHGJ
DCfMVDFVScVMVQlgmZgdmWQQmS
nWTWWgwNgGDdBZBVcvDzzJccVlCzHD
RLppMMLpRqfMtMjtMCHJFSpzHSvSpczJdl
RdLRbQRjsRMrMRRMfbQLqPjbmnQgQWWwZmggnNTgnnWwhBhn
TmzjMjrmjmjBmHLvGPpbvWGqJzJJ
CfScwNDssDVCccdNVcNDQfbqJLqSLPpJpJJvJPpGTWJb
nddCQTfQVVrHmjMnrMFM
WHDnTwvwcwZmWwQTnBtgbVLLbgfSlTfrfb
CPPGdJRzNhNpzPJtMgBLbgtlLLfLMz
GJptpdFRhJwDmFHDjvnD
PNcWDNnNDcLjDDcSRWtQFfzzzQgPgsssZtPZ
MGhJRJGGrlpVGVHVCqqGqBQvzFzFfBvZvvtZtvFzvZvQ
JmJplCrlMNdmjbNbWR
pqpqFJPPSswJshNghg
zTHHrrbLzDQHccfhqmDshgCwfmqm
rtqqtTTHtzGLPWBdnGBSWGSS
gmNvgVqjjqzfMRgrRtPcft
WswSQwWWHGCsHQhlGGLLJRbLMRfRGcMb
dQCRCWhhwCwFwQshhRTmmBmFjFTTVNpqTnTj
FZvqSWqjjZvvrNSvbblcbslDppDHbcsS
MmwLTwPmBwmLJJmLlWzWnDllHcHzcDHM
QRwtQtLTwwQBTPfFfZdFvqqrFGWjrh
MqlnnNvJJZnNNdJZZLvLJnMzjjCTCChgWjccWqcjhcgcWR
tbddSHDBbHgHhHTCjh
mffBfbpsFSdQQDbQsrlJvwJNLLJMrrlsJr
RjqbNRRbDDqHndbcHDqdRHcChsffCQJJssZGpzpCpJphJd
VrMmSbrWVMtMtLmQGCfZWJZCfpZfJW
PgSPgTvSSHjjBbvbvq
whclNQQfcCFCcrJRjmmHNWsmLs
PPzMbqBPLVtnTppPTPzHJrGWJRRvsjjjRHssRb
PtgPVZTtTLMtBzqPqttPVMClcdDcgCCfCQDSdSgCwlhh
DrcrsvcNtLWSFPSFszbM
HqTdHJdQhGJBHQHWWDTnnPzbMMzPnS
hdwfqdhQJfjlccrfvNDLfl
BBPCWvjvTLrHTHHPCTndfwhbdnnZZfDhJwfJVb
msmgNMMcgmgczlmmgQNlddpVDDZpZpDfhZfJwsJF
mmQmMgSgzmqSRllGmgjPHrvvwHttjLrLRWWB
RwvDvhjhMvwlFNwNwCWCCWWLZcbGGZLGJVrppbZVcjmpmJ
tfnfsStnPPfTfgnPSSzPflJZZZrlZJVzLLmZJrzVmG
nPHqgfsHQnffqgSTldHTPnPHWMRFFqqFhNNNCwNCFMMhDMhh
FmwFHmnlGJfnlSlmrfsSvWgZNWNvLvtqLqDJhWJD
BVVTTMqRWvRZRbhW
pMBPCzVPQcQsGqSFmPFwSF
TcpTpwqZqMpZqlZCpZlwDjjcPNdgdPjHHHdvhHQgvv
zQFBRbmsQbLLBnGBvFdPghddNgHjNSSj
QnszWVzLfsLGbnGQbwpVtMrwZTqpCqpppr
WrZmrJcGwZdGZZmHdJcwGWcZsdFFLqTtLVtSTLtvvLtLLqSs
fnpCCQClfpQlzbbpQpflBpjhLVSTvFhjqtstVsjtstFVMs
QBQRnbCRpnPngnbggCzzRClZwJHFHDwZJPJGWHwmNGHPZr
fDhjvftQtDwgPhdRcRRP
bbNSgllVNMCWVnbWmcdFdmmFdpFpRrPPPw
NBzBglNzBvvGZDJQ
rGbbtStjSdbGtDpjjJbbRRbdrcCsCCrFqhllrFHsFsCvqCWH
TzgMMgmTVgzzTMLLfMHvTFsFCqHTvFsTvshF
gLMPBgQmQmNzVZLPzPppjbRhhddGBGSttbpR
zMJTpMzpVczHbCzVJVFCpJPngnBqVZqsRZZPnjqRgmjR
wttwNdfLQwLhwhhDDhWvgRmnPqsQqjnBPSZRgjPS
DdLwGGvNvhlvrrMFlFTcZrpC
llBQWMScQlSSBjMrvrrPpFHFBDFDFJTmTtFFmF
nzZzfVgzCNtnJppDHPnPpp
LdRNfVdzbzCCjcvllMcsbtjj
pWFwpFhprTnFfWwZrsBDmsqBvZvjjv
VbcHCthtzQtNqBsvsZvQmQsj
cltzzVcJSMtRWdhJLhRwdh
lmmmLRdZnjBlGgVhNSVvRMWN
bDwCqCGPbwpPwDPPpCpqpPbScvMSMSDWgVcSShNNVfgWWv
bTpzqJHPFFJqbTHswLlGlBntGnjQtGBZTT
pVjVlDDhmRPlHlHPWzWVWrVrcWztVdzv
GCqGGGJSFbnLJLLfrLfPrLdgrrgfWd
PbJSQGSsGnbMbqSFGBMMbsGTNwpTRwppDRsjwlNpsmHwDl
GcnPbbbLqDPDBPPDlQ
JtTChNfRpNJMTCfMTlJVsdVHvDddHBVmQm
TzlffWNfjTfjjCjfTtRrLgbnbbnSSwbnLzZZzr
hggWzjLhzhLhjzVWgpCpTFFHtCJFTJTHHHdG
lvSBvNNSNSSmlbwmMJCGtJQCHmdT
cfSlSSlvBDBPnlPPDZLsgtggZZVVfhzRgV
gGVJGvVVZZLvQLWQppccpctpNptMhnhSjjnM
fzQBsBzmwPzdQrRbSFNFnsSDchjnFhMN
bCwQRbbCJvlGCHgL
NMgdHVSqgQcVHmlllLDjlCLdjL
whTRJtJTnthWBlLLLmlFtVmV
wRhnGTpzzTRnVbzzWWbJwbhNQrZHfpNgMQpfZQHHNZZHHQ
qNNlMdbNrlVsQQfswQNCmW
LJzBvSdLSHpDJzzzHJnHnzFQCCmmswmfwBGhsQfWfCwW
HzSvHppDDRvgHzzcnqTrTtllZdRrbRVVZZ
TWVVVFVPpjVFtRfPBmmzMMPCvmLm
hDDrwndQQbbhZDMSMvcflMLmfnBL
grdggqsbrhdJJJrhbwbbsZGHVNHtTWpVBFVTTTVTHtNg
tLbvnTCzCVnzzwVTJVlwltMFvQFQRFFrQPBFdNgrMBNF
pShsqqmGjZfZccsqSfbdNBMMRQGBPgMdPMPF
HhspDpjhSjbcSmcqhZDcZZjTttLVwlTJJVWtnWVlWHCltL
DwQBvwBnBrSVRrZM
JWWGRssgRsFgzsFPJrNHVMHrlVrPSMNjHH
gspgFzRCFWLJpgqqRWgqpCJwnQCnhQvwcTdcdddnwbDfhf
fMMCwFDGNNMTdTDLlVlZZmdZBdBtVr
jjpjtbpRcPvSPnPnpPnvPPPnbZlmrWmLWHmHBWHlrmrmlZWl
jqjjnPqngtQfGFftFq
qnzhhbzzqGgsqGtnwcJrlCMlCjvcCCcrCRrvCv
mVSNZdTQdVVWmVFHQrpCjpjDRvjMRjdLRt
WBHZTHHTFWWNNBNHQTZPsPggghfgsnsgsffthszJ
sDwpdMgvHrZgwbdggzZZgPhFNFFcjPPqhLhjMlPqLq
JffJfnJGtBtCQQRBJZTQJJGfcNjqCLcFhFWCPLjhFLPjcPhl
VZZVmtGQQZmHrwgddppb
NNNlpjbVpGglNbvpTwBQQvfWvfBrWvBW
JthDJsLhhHPcGcqPshJsshSBwCWwfWSLCfSfSSRBrfWB
dhDtZqGmctZDtZHqDGVgVgNbbbFjndMFNlFg
jqDVzzDMdDwsVQLCZVCRWLGBQC
bbHFbSSpFbFHJHStJNbtJprZlCGLQsLGZRBLRQLQpLBClZ
SmFmTPJvFTNbmmsMdqqjscwz
RqMbHGJRJpgJgGgQjgrLTrTzjcrTrrLg
lwfwdhnflPlbCsVVjhCSbV
fwnNtwmlFZpbFZtM
QNSQrLTNrLNQRRrfHFrSjqhblWtSltqlWqtWWl
DvgBgnzgcvVZMljv
DGJdjBPDngDnDjBpBmFpTRmRFLRRmmCmHH
dtgSdTqdlvdJJvFqTvSqJqqRMPBtLHPBnWbsbWbsbWtWtWHV
DjfCfmcpNrwZQCmmNrDZNZNpsHLHWBBbHVVcGGGbHGBbMVMB
fpNjCfzLNZjTllvzdSgFgJ
nHBfZmHTRwsZffjBnHfmRZHtLtdNPPlWvddWLWJlJldCldLC
zwrbphMMVFzMwdCWNPlCbPdDCD
hccMSpqShMSrhZTBwmTqHjqZmf
NJGGLwGsTSsNlJZhmtgCVlbWQWlQMtCbvb
pDjppDBRdjzqpHqDSDzjVMCCrCrWtgQWdtdQQCdb
fHpfRBPRzDpBFSqcSBRBSNhPNwwNNswJLhmmLNGJZL
RZbsPgnVDzTJcdGjDGmD
NwfQHQtpfppQhjVVjqVNJJTN
tLLtwSptVCSWpvVtRnrbWbMgFPMsgZss
VsQjSwwVSwsZzsvdscZvfrhPRpqBrBrbpzMrrTMh
JtNDTmtGJTmLCGFDCHtDhMMBRhqqRbPrfrbNRrbR
GDWLlDnFgZdTSTWv
gfQHRRpQgRqNSBtjqwjztzjtnL
FvsPgvDlFcmvmTLlBMVztnLwWLzL
cvZGDcvbPcmbTbrcDDPgvbTJdJfJHRhZSRSpRRfHdSpSZd
FGHHHWvBWrHHrWVZJvVtcSrtrTcrtcjMTjStSM
swpwfRhRmmmRQDzcJlcqMSMMqtbMTz
dQDRDDRQfmmQDNsLfwRJvZJWFWVnPWvvPddBnC
zVhHvhMVTnMJgcTzMcfGJtfBWRtBRqJWtqJb
hlZjZSQCZtfBbGjqbb
SQDNwplrDVnhDcVHgT
HHZmttZVLVMQQJwMfcDRfDbbMN
phWvTgBqqgBWsTPTzhWzhzfNGNNwfqfcDwJRRbNcJcbl
FnTsvnhppTPFTBpPzPvpBBpFVmtfCjfmjtCmFdStLmtdfjSd
hMTPPMNlLZNSGNbRBB
gjmrsrttsCnsCsttVsjvttvtZGFHdbZGWWWdZSWddBHHnSWH
BmvptjsrfjBgsvvfgmfQchMpMqqhcQPMMPMhLQ
dJHGnrJJpGpDpFzzDmfsfSSCbfTfMbbRDb
vLcwtWNgtVtSNWmTmTMCmhNhTRfM
jqVgqVvWwwLwwgqZgjVSrPGzHHHqFnrHnFGHJGdr
QcQcdgbzwJnzfgVnVwdHQbdBrrpplHvmhLjrlTphppLrjl
MsPssfSZMWGWqWssSNPqrmmLBvjhqTvhjBprhB
GRDDZMFNMGFCSNWFPDMMFWNnwVtdVdQfcgtddwQwzRJfQd
pMqCMBGpMMCnwnpBMGBlbVRFJFWsJzRdzHlWzzJdJsJd
jLLZjDgPbNPZTDbvftvZgzRFsRzRrRRHWFsJsFJc
TZZLDPjtmDmbqQGwQnVM
RjnNhBnnNNmJmBNhrqGpsHbHtstgTtTjqb
WVMfWwQTzWZDMtlsfldqpplstf
WVTWSTzwQWSSvQVZCQPTSZPvBcnmcvnrhmNcRFncNLRLJmnB
lffQcnNTQBBcwqsdcqjSspcWtD
MPMGrHGMMTqWTtDg
VGRGLrJHZzLHPzTNTmmBfZQFTNBQ
lQBPPrBrlnqBgSRhgZZZrLJr
VNcWMDZbJHhWfWff
VwvMwMvdwvdwjcwdwDDqsqsZQqPPzqzldPQtps
VVJcZJVrRSdcsddfsvvbvFZBnngBMzMZFD
hLLqLpqqWWphWjhlGlCHDFgzbvFBbgBFtnQpnngg
HlhlmmHBTqTHHmlLqjTGhHhPdwdmsdcSNSfNJRrRcfwVJdrr
HmhvmRzzHHrRMBJBjNJGDJRnJD
cbqcPqLWZwsgQWZwWPwWgPPbntJnrNftNNNBtNJJNDBNqdjB
lgwWgggQcWSzrlTHMHlp
FZhQpClCWLBlrNDZvrwrrNSH
ztTfjMjJjgsGrqvzDGwG
bMjfnjndjnJtfgMTwPjFhQhCLdQCFWQQLWQhch
PtrqPrrMCPChzCtLFRbtNgbdTjbF
GdZGvJSWWDGplFjbjLjLSTlL
vspvnZsVppBdBnBpDCszhzMsMzzPzPhcPC
PdCtdfCMfGmtfmtBSWrMQwSlwWwJNQ
qTqbcjqHTZTbcDqczTjjZvWrQvVWJQSVQZZSVVrJNr
qbqjRFTcHTcDFgcczRqFrPsGGGgnhtGssCdnffGmsP
vVbcMvqjjjmHCBCBBmBSSH
PzzLTrcrwQgfLGrJBHSGFSCHtRFBJt
cfsPrLDwQNgsrTNTQgLwVpWnVWvMNvqjjqvpMvlj
jbqZNjLbZQvcfhhQ
CWdCDWWMCgmJDnJmdQVzMSchvfcQVFShzf
WPgWGRWRHdPBsttrHvNtrl
MCJCCMCqcCqJsnssjQRlTvQQQQqTvqfQ
HGVmwmLVlZRzcGZG
FcchHmDFpFbDMDMbntsJ
vfNvvttvDRcrvRNRNTLDNRcVsFhwVBVTwbbFFVbVhbJMBB
CJGgSJHdgPPSnQnjnddHGGFMSMwMMsmsFMMFhFShMWMM
dGCdjgzHCPPGlHDDqJtqRcqJlpfR
CwtqqvwLwnwhtRLtdRnwnnRhPHpLLppTPPpTVfTHzJWVLTWB
sGDsZDllDrMFZVmGgsmDMlmHpJBHpcJFPPTHfJJPzfTHWz
srgVlGSgZVGGrRvwQwdqwtCvnS
MLPgDrgLzThhCTgg
GVfCbvVbVvhBHlmhvhHv
ZqRQffRwtNZWVZfZSMsSssncnDCDnL
VwBNhNNmhJswqjJsRzPgDvSgJvtgvgDt
rRMblbHFfRrSWvPPPgMzLW
ZFpFpCcprGfbrGfGCHclZfnGqmsjmBdNVjQqRBnqBsVNQwBh
VLQlZWQbcPgcPmWbgmDWLQzdpGMTTqdpMTNsbGsNpMSqdq
HChrwfffhJtfHwJTsDqThhDMpTGsjj
HvffCrtBzclQZvQD
dCBVJJmrJDlBdQJWZTTsWbdWThWpbM
FHjSPjwqwssSgqTMGbGWhTMHMMZG
FwLvLfLjjSPqFqgzwffFSvrlDJvrQrDVrnmBvrJsQm
hZCRbddrgrRSVgcGZjjLFGLZFQFp
nzPvMsPvtmvPNssPMqpcWVjGGcGLlqGcGN
TBzmTmzmVmrBSBRrRgdR
bwZZmwfFmcfCRswNWNBsjC
VDSdglSStRCCTNCD
VVVppGHGlrQnnGJbMmnmCh
nFhgnFVNtgtFVssgdgVtVtsqJPTNJvDSNqTZZzbzTDDzDq
HqLHqcwlBjLzPvPJCCvJ
HGrlHqlllHBppRrlwWFnnWfFFVhMnnWdFhfn
FsMFVszjggMMzWlPJlPPlLQsJv
nqnBSNlfZqSvLHnHvWLQTW
hShpfCCSRtfjgFjMzdjlpr
jsbDsQnnwPsFbZgSltWGdgJgpnSp
SCVvLhLRrzCNBhrCvddMJpWdWGvJGlgtpv
cHLBBVCcCNHrbcSQFwQTbDws
rMlbLgrRgpwTDbflcnHtSScwHdJdwHJB
CQCjjZPzGGzCzZQvBVBNdQNSJVcJ
jsPhCzhPqFZzZzChFlsbRRLrMfTbbcLTpD
dqjzmmmQBjBHCmWrgVGWrrrvrpgVpM
PLJnTFbJnhTDzrGgMlvrFMzF
SssPcDttntDSsLcCzHBmmwcmZQqH
fFfFSPHllPRpRfnmmFfHvHjgbsBQjsptBdBTTddjsDdt
ZqLJCLCZzzTgWjdzDjzb
ZhNLMrNcGrchLqcCVMqRvlSnFnRnmmGRggnPnP
HMCMCMrHfwMHtFwTtgHHbVjjbsRZDwDVRbZqjhBD
cDpmpdpNLNDcdZjZqZBNBqssRR
SLJSdPcznSvdvDcHFCftFTFWCTHnWt
NlMmlPClmdGldRZHJs
tgJJcJQcntHBsnBt
TfSgQhhccczSVQqrgSgTjFWqLWmwLFNJLWWPPwqM
GGwTHqWVdnTTVVqgngzzqHzGFbJspFccRsjDpDcjsRFDsdjR
rtLflllZSrhQPLBtQStZvhjDNjbcDNDRNFjCsCDCFs
mPLBQPtvtQZtBlLSmMqVGwHbVMqTHWmz
VvJCrqTvPvQrCpRNVRFGfZfmfG
HMzsdWsjhZSsJJZNZf
wHlbjnlzWCvqJBQlrD
FnVRRsVdSnSnFSRqTVdqBBDBhrDdmcddMcMQMhzm
HtZvJwHlgjlGlHJLNjJrMmrcmDQfDLczDrMhhh
vGGpJjttvlZljZllZvJZvwGqsSTRWSCpnCWTVPVmWWTWWn
wSHCNwwmcSMLSDFcwwSSHQvZnQjLZffZjZZbVZjVVb
JqsNJJGGqprJNtpWhGhspfnffTnTvZvVbZnTrfBQvV
GsWhdGtPWpghJRqhtNPmClczSlDglHMlczmwCH
TzRpjVRjFpVLTTdgrTgrGsZwrZZwgg
vQfSBdbDbMbQNBJrlhmGnrgrgwZhvm
SHSCbdbddcVWqqFPCLqR";
