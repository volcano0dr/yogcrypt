use ::*;

const SM4_BLOCK_SIZE: usize = 16;
const SM4_KEY_SIZE: usize = 16;
const SM4_RND_KEY_SIZE: u32 = 32 * 4;

const Sbox: [u8;256] = [
	0xD6, 0x90, 0xE9, 0xFE, 0xCC, 0xE1, 0x3D, 0xB7, 0x16, 0xB6, 0x14, 0xC2, 0x28, 0xFB, 0x2C, 0x05,
	0x2B, 0x67, 0x9A, 0x76, 0x2A, 0xBE, 0x04, 0xC3, 0xAA, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99,
	0x9C, 0x42, 0x50, 0xF4, 0x91, 0xEF, 0x98, 0x7A, 0x33, 0x54, 0x0B, 0x43, 0xED, 0xCF, 0xAC, 0x62,
	0xE4, 0xB3, 0x1C, 0xA9, 0xC9, 0x08, 0xE8, 0x95, 0x80, 0xDF, 0x94, 0xFA, 0x75, 0x8F, 0x3F, 0xA6,
	0x47, 0x07, 0xA7, 0xFC, 0xF3, 0x73, 0x17, 0xBA, 0x83, 0x59, 0x3C, 0x19, 0xE6, 0x85, 0x4F, 0xA8,
	0x68, 0x6B, 0x81, 0xB2, 0x71, 0x64, 0xDA, 0x8B, 0xF8, 0xEB, 0x0F, 0x4B, 0x70, 0x56, 0x9D, 0x35,
	0x1E, 0x24, 0x0E, 0x5E, 0x63, 0x58, 0xD1, 0xA2, 0x25, 0x22, 0x7C, 0x3B, 0x01, 0x21, 0x78, 0x87,
	0xD4, 0x00, 0x46, 0x57, 0x9F, 0xD3, 0x27, 0x52, 0x4C, 0x36, 0x02, 0xE7, 0xA0, 0xC4, 0xC8, 0x9E,
	0xEA, 0xBF, 0x8A, 0xD2, 0x40, 0xC7, 0x38, 0xB5, 0xA3, 0xF7, 0xF2, 0xCE, 0xF9, 0x61, 0x15, 0xA1,
	0xE0, 0xAE, 0x5D, 0xA4, 0x9B, 0x34, 0x1A, 0x55, 0xAD, 0x93, 0x32, 0x30, 0xF5, 0x8C, 0xB1, 0xE3,
	0x1D, 0xF6, 0xE2, 0x2E, 0x82, 0x66, 0xCA, 0x60, 0xC0, 0x29, 0x23, 0xAB, 0x0D, 0x53, 0x4E, 0x6F,
	0xD5, 0xDB, 0x37, 0x45, 0xDE, 0xFD, 0x8E, 0x2F, 0x03, 0xFF, 0x6A, 0x72, 0x6D, 0x6C, 0x5B, 0x51,
	0x8D, 0x1B, 0xAF, 0x92, 0xBB, 0xDD, 0xBC, 0x7F, 0x11, 0xD9, 0x5C, 0x41, 0x1F, 0x10, 0x5A, 0xD8,
	0x0A, 0xC1, 0x31, 0x88, 0xA5, 0xCD, 0x7B, 0xBD, 0x2D, 0x74, 0xD0, 0x12, 0xB8, 0xE5, 0xB4, 0xB0,
	0x89, 0x69, 0x97, 0x4A, 0x0C, 0x96, 0x77, 0x7E, 0x65, 0xB9, 0xF1, 0x09, 0xC5, 0x6E, 0xC6, 0x84,
	0x18, 0xF0, 0x7D, 0xEC, 0x3A, 0xDC, 0x4D, 0x20, 0x79, 0xEE, 0x5F, 0x3E, 0xD7, 0xCB, 0x39, 0x48];

const FK: [u32;4] = [0xa3b1bac6, 0x56aa3350, 0x677d9197, 0xb27022dc];

const CK: [u32;32] = [
	0x00070E15, 0x1C232A31, 0x383F464D, 0x545B6269,
	0x70777E85, 0x8C939AA1, 0xA8AFB6BD, 0xC4CBD2D9,
	0xE0E7EEF5, 0xFC030A11, 0x181F262D, 0x343B4249,
	0x50575E65, 0x6C737A81, 0x888F969D, 0xA4ABB2B9,
	0xC0C7CED5, 0xDCE3EAF1, 0xF8FF060D, 0x141B2229,
	0x30373E45, 0x4C535A61, 0x686F767D, 0x848B9299,
	0xA0A7AEB5, 0xBCC3CAD1, 0xD8DFE6ED, 0xF4FB0209,
	0x10171E25, 0x2C333A41, 0x484F565D, 0x646B7279
	];

const Sbox_T: [u32;256] = [
	0x8ED55B5B, 0xD0924242, 0x4DEAA7A7, 0x06FDFBFB, 0xFCCF3333, 0x65E28787,
	0xC93DF4F4, 0x6BB5DEDE, 0x4E165858, 0x6EB4DADA, 0x44145050, 0xCAC10B0B,
	0x8828A0A0, 0x17F8EFEF, 0x9C2CB0B0, 0x11051414, 0x872BACAC, 0xFB669D9D,
	0xF2986A6A, 0xAE77D9D9, 0x822AA8A8, 0x46BCFAFA, 0x14041010, 0xCFC00F0F,
	0x02A8AAAA, 0x54451111, 0x5F134C4C, 0xBE269898, 0x6D482525, 0x9E841A1A,
	0x1E061818, 0xFD9B6666, 0xEC9E7272, 0x4A430909, 0x10514141, 0x24F7D3D3,
	0xD5934646, 0x53ECBFBF, 0xF89A6262, 0x927BE9E9, 0xFF33CCCC, 0x04555151,
	0x270B2C2C, 0x4F420D0D, 0x59EEB7B7, 0xF3CC3F3F, 0x1CAEB2B2, 0xEA638989,
	0x74E79393, 0x7FB1CECE, 0x6C1C7070, 0x0DABA6A6, 0xEDCA2727, 0x28082020,
	0x48EBA3A3, 0xC1975656, 0x80820202, 0xA3DC7F7F, 0xC4965252, 0x12F9EBEB,
	0xA174D5D5, 0xB38D3E3E, 0xC33FFCFC, 0x3EA49A9A, 0x5B461D1D, 0x1B071C1C,
	0x3BA59E9E, 0x0CFFF3F3, 0x3FF0CFCF, 0xBF72CDCD, 0x4B175C5C, 0x52B8EAEA,
	0x8F810E0E, 0x3D586565, 0xCC3CF0F0, 0x7D196464, 0x7EE59B9B, 0x91871616,
	0x734E3D3D, 0x08AAA2A2, 0xC869A1A1, 0xC76AADAD, 0x85830606, 0x7AB0CACA,
	0xB570C5C5, 0xF4659191, 0xB2D96B6B, 0xA7892E2E, 0x18FBE3E3, 0x47E8AFAF,
	0x330F3C3C, 0x674A2D2D, 0xB071C1C1, 0x0E575959, 0xE99F7676, 0xE135D4D4,
	0x661E7878, 0xB4249090, 0x360E3838, 0x265F7979, 0xEF628D8D, 0x38596161,
	0x95D24747, 0x2AA08A8A, 0xB1259494, 0xAA228888, 0x8C7DF1F1, 0xD73BECEC,
	0x05010404, 0xA5218484, 0x9879E1E1, 0x9B851E1E, 0x84D75353, 0x00000000,
	0x5E471919, 0x0B565D5D, 0xE39D7E7E, 0x9FD04F4F, 0xBB279C9C, 0x1A534949,
	0x7C4D3131, 0xEE36D8D8, 0x0A020808, 0x7BE49F9F, 0x20A28282, 0xD4C71313,
	0xE8CB2323, 0xE69C7A7A, 0x42E9ABAB, 0x43BDFEFE, 0xA2882A2A, 0x9AD14B4B,
	0x40410101, 0xDBC41F1F, 0xD838E0E0, 0x61B7D6D6, 0x2FA18E8E, 0x2BF4DFDF,
	0x3AF1CBCB, 0xF6CD3B3B, 0x1DFAE7E7, 0xE5608585, 0x41155454, 0x25A38686,
	0x60E38383, 0x16ACBABA, 0x295C7575, 0x34A69292, 0xF7996E6E, 0xE434D0D0,
	0x721A6868, 0x01545555, 0x19AFB6B6, 0xDF914E4E, 0xFA32C8C8, 0xF030C0C0,
	0x21F6D7D7, 0xBC8E3232, 0x75B3C6C6, 0x6FE08F8F, 0x691D7474, 0x2EF5DBDB,
	0x6AE18B8B, 0x962EB8B8, 0x8A800A0A, 0xFE679999, 0xE2C92B2B, 0xE0618181,
	0xC0C30303, 0x8D29A4A4, 0xAF238C8C, 0x07A9AEAE, 0x390D3434, 0x1F524D4D,
	0x764F3939, 0xD36EBDBD, 0x81D65757, 0xB7D86F6F, 0xEB37DCDC, 0x51441515,
	0xA6DD7B7B, 0x09FEF7F7, 0xB68C3A3A, 0x932FBCBC, 0x0F030C0C, 0x03FCFFFF,
	0xC26BA9A9, 0xBA73C9C9, 0xD96CB5B5, 0xDC6DB1B1, 0x375A6D6D, 0x15504545,
	0xB98F3636, 0x771B6C6C, 0x13ADBEBE, 0xDA904A4A, 0x57B9EEEE, 0xA9DE7777,
	0x4CBEF2F2, 0x837EFDFD, 0x55114444, 0xBDDA6767, 0x2C5D7171, 0x45400505,
	0x631F7C7C, 0x50104040, 0x325B6969, 0xB8DB6363, 0x220A2828, 0xC5C20707,
	0xF531C4C4, 0xA88A2222, 0x31A79696, 0xF9CE3737, 0x977AEDED, 0x49BFF6F6,
	0x992DB4B4, 0xA475D1D1, 0x90D34343, 0x5A124848, 0x58BAE2E2, 0x71E69797,
	0x64B6D2D2, 0x70B2C2C2, 0xAD8B2626, 0xCD68A5A5, 0xCB955E5E, 0x624B2929,
	0x3C0C3030, 0xCE945A5A, 0xAB76DDDD, 0x867FF9F9, 0xF1649595, 0x5DBBE6E6,
	0x35F2C7C7, 0x2D092424, 0xD1C61717, 0xD66FB9B9, 0xDEC51B1B, 0x94861212,
	0x78186060, 0x30F3C3C3, 0x897CF5F5, 0x5CEFB3B3, 0xD23AE8E8, 0xACDF7373,
	0x794C3535, 0xA0208080, 0x9D78E5E5, 0x56EDBBBB, 0x235E7D7D, 0xC63EF8F8,
	0x8BD45F5F, 0xE7C82F2F, 0xDD39E4E4, 0x68492121 ];

static Sbox_T8: [u32;256] =
[
0x5b8ed55b, 0x42d09242, 0xa74deaa7, 0xfb06fdfb,
0x33fccf33, 0x8765e287, 0xf4c93df4, 0xde6bb5de,
0x584e1658, 0xda6eb4da, 0x50441450, 0x0bcac10b,
0xa08828a0, 0xef17f8ef, 0xb09c2cb0, 0x14110514,
0xac872bac, 0x9dfb669d, 0x6af2986a, 0xd9ae77d9,
0xa8822aa8, 0xfa46bcfa, 0x10140410, 0x0fcfc00f,
0xaa02a8aa, 0x11544511, 0x4c5f134c, 0x98be2698,
0x256d4825, 0x1a9e841a, 0x181e0618, 0x66fd9b66,
0x72ec9e72, 0x094a4309, 0x41105141, 0xd324f7d3,
0x46d59346, 0xbf53ecbf, 0x62f89a62, 0xe9927be9,
0xccff33cc, 0x51045551, 0x2c270b2c, 0x0d4f420d,
0xb759eeb7, 0x3ff3cc3f, 0xb21caeb2, 0x89ea6389,
0x9374e793, 0xce7fb1ce, 0x706c1c70, 0xa60daba6,
0x27edca27, 0x20280820, 0xa348eba3, 0x56c19756,
0x02808202, 0x7fa3dc7f, 0x52c49652, 0xeb12f9eb,
0xd5a174d5, 0x3eb38d3e, 0xfcc33ffc, 0x9a3ea49a,
0x1d5b461d, 0x1c1b071c, 0x9e3ba59e, 0xf30cfff3,
0xcf3ff0cf, 0xcdbf72cd, 0x5c4b175c, 0xea52b8ea,
0x0e8f810e, 0x653d5865, 0xf0cc3cf0, 0x647d1964,
0x9b7ee59b, 0x16918716, 0x3d734e3d, 0xa208aaa2,
0xa1c869a1, 0xadc76aad, 0x06858306, 0xca7ab0ca,
0xc5b570c5, 0x91f46591, 0x6bb2d96b, 0x2ea7892e,
0xe318fbe3, 0xaf47e8af, 0x3c330f3c, 0x2d674a2d,
0xc1b071c1, 0x590e5759, 0x76e99f76, 0xd4e135d4,
0x78661e78, 0x90b42490, 0x38360e38, 0x79265f79,
0x8def628d, 0x61385961, 0x4795d247, 0x8a2aa08a,
0x94b12594, 0x88aa2288, 0xf18c7df1, 0xecd73bec,
0x04050104, 0x84a52184, 0xe19879e1, 0x1e9b851e,
0x5384d753, 0x00000000, 0x195e4719, 0x5d0b565d,
0x7ee39d7e, 0x4f9fd04f, 0x9cbb279c, 0x491a5349,
0x317c4d31, 0xd8ee36d8, 0x080a0208, 0x9f7be49f,
0x8220a282, 0x13d4c713, 0x23e8cb23, 0x7ae69c7a,
0xab42e9ab, 0xfe43bdfe, 0x2aa2882a, 0x4b9ad14b,
0x01404101, 0x1fdbc41f, 0xe0d838e0, 0xd661b7d6,
0x8e2fa18e, 0xdf2bf4df, 0xcb3af1cb, 0x3bf6cd3b,
0xe71dfae7, 0x85e56085, 0x54411554, 0x8625a386,
0x8360e383, 0xba16acba, 0x75295c75, 0x9234a692,
0x6ef7996e, 0xd0e434d0, 0x68721a68, 0x55015455,
0xb619afb6, 0x4edf914e, 0xc8fa32c8, 0xc0f030c0,
0xd721f6d7, 0x32bc8e32, 0xc675b3c6, 0x8f6fe08f,
0x74691d74, 0xdb2ef5db, 0x8b6ae18b, 0xb8962eb8,
0x0a8a800a, 0x99fe6799, 0x2be2c92b, 0x81e06181,
0x03c0c303, 0xa48d29a4, 0x8caf238c, 0xae07a9ae,
0x34390d34, 0x4d1f524d, 0x39764f39, 0xbdd36ebd,
0x5781d657, 0x6fb7d86f, 0xdceb37dc, 0x15514415,
0x7ba6dd7b, 0xf709fef7, 0x3ab68c3a, 0xbc932fbc,
0x0c0f030c, 0xff03fcff, 0xa9c26ba9, 0xc9ba73c9,
0xb5d96cb5, 0xb1dc6db1, 0x6d375a6d, 0x45155045,
0x36b98f36, 0x6c771b6c, 0xbe13adbe, 0x4ada904a,
0xee57b9ee, 0x77a9de77, 0xf24cbef2, 0xfd837efd,
0x44551144, 0x67bdda67, 0x712c5d71, 0x05454005,
0x7c631f7c, 0x40501040, 0x69325b69, 0x63b8db63,
0x28220a28, 0x07c5c207, 0xc4f531c4, 0x22a88a22,
0x9631a796, 0x37f9ce37, 0xed977aed, 0xf649bff6,
0xb4992db4, 0xd1a475d1, 0x4390d343, 0x485a1248,
0xe258bae2, 0x9771e697, 0xd264b6d2, 0xc270b2c2,
0x26ad8b26, 0xa5cd68a5, 0x5ecb955e, 0x29624b29,
0x303c0c30, 0x5ace945a, 0xddab76dd, 0xf9867ff9,
0x95f16495, 0xe65dbbe6, 0xc735f2c7, 0x242d0924,
0x17d1c617, 0xb9d66fb9, 0x1bdec51b, 0x12948612,
0x60781860, 0xc330f3c3, 0xf5897cf5, 0xb35cefb3,
0xe8d23ae8, 0x73acdf73, 0x35794c35, 0x80a02080,
0xe59d78e5, 0xbb56edbb, 0x7d235e7d, 0xf8c63ef8,
0x5f8bd45f, 0x2fe7c82f, 0xe4dd39e4, 0x21684921,
];

static Sbox_T16: [u32;256] =
[
0x5b5b8ed5, 0x4242d092, 0xa7a74dea, 0xfbfb06fd,
0x3333fccf, 0x878765e2, 0xf4f4c93d, 0xdede6bb5,
0x58584e16, 0xdada6eb4, 0x50504414, 0x0b0bcac1,
0xa0a08828, 0xefef17f8, 0xb0b09c2c, 0x14141105,
0xacac872b, 0x9d9dfb66, 0x6a6af298, 0xd9d9ae77,
0xa8a8822a, 0xfafa46bc, 0x10101404, 0x0f0fcfc0,
0xaaaa02a8, 0x11115445, 0x4c4c5f13, 0x9898be26,
0x25256d48, 0x1a1a9e84, 0x18181e06, 0x6666fd9b,
0x7272ec9e, 0x09094a43, 0x41411051, 0xd3d324f7,
0x4646d593, 0xbfbf53ec, 0x6262f89a, 0xe9e9927b,
0xccccff33, 0x51510455, 0x2c2c270b, 0x0d0d4f42,
0xb7b759ee, 0x3f3ff3cc, 0xb2b21cae, 0x8989ea63,
0x939374e7, 0xcece7fb1, 0x70706c1c, 0xa6a60dab,
0x2727edca, 0x20202808, 0xa3a348eb, 0x5656c197,
0x02028082, 0x7f7fa3dc, 0x5252c496, 0xebeb12f9,
0xd5d5a174, 0x3e3eb38d, 0xfcfcc33f, 0x9a9a3ea4,
0x1d1d5b46, 0x1c1c1b07, 0x9e9e3ba5, 0xf3f30cff,
0xcfcf3ff0, 0xcdcdbf72, 0x5c5c4b17, 0xeaea52b8,
0x0e0e8f81, 0x65653d58, 0xf0f0cc3c, 0x64647d19,
0x9b9b7ee5, 0x16169187, 0x3d3d734e, 0xa2a208aa,
0xa1a1c869, 0xadadc76a, 0x06068583, 0xcaca7ab0,
0xc5c5b570, 0x9191f465, 0x6b6bb2d9, 0x2e2ea789,
0xe3e318fb, 0xafaf47e8, 0x3c3c330f, 0x2d2d674a,
0xc1c1b071, 0x59590e57, 0x7676e99f, 0xd4d4e135,
0x7878661e, 0x9090b424, 0x3838360e, 0x7979265f,
0x8d8def62, 0x61613859, 0x474795d2, 0x8a8a2aa0,
0x9494b125, 0x8888aa22, 0xf1f18c7d, 0xececd73b,
0x04040501, 0x8484a521, 0xe1e19879, 0x1e1e9b85,
0x535384d7, 0x00000000, 0x19195e47, 0x5d5d0b56,
0x7e7ee39d, 0x4f4f9fd0, 0x9c9cbb27, 0x49491a53,
0x31317c4d, 0xd8d8ee36, 0x08080a02, 0x9f9f7be4,
0x828220a2, 0x1313d4c7, 0x2323e8cb, 0x7a7ae69c,
0xabab42e9, 0xfefe43bd, 0x2a2aa288, 0x4b4b9ad1,
0x01014041, 0x1f1fdbc4, 0xe0e0d838, 0xd6d661b7,
0x8e8e2fa1, 0xdfdf2bf4, 0xcbcb3af1, 0x3b3bf6cd,
0xe7e71dfa, 0x8585e560, 0x54544115, 0x868625a3,
0x838360e3, 0xbaba16ac, 0x7575295c, 0x929234a6,
0x6e6ef799, 0xd0d0e434, 0x6868721a, 0x55550154,
0xb6b619af, 0x4e4edf91, 0xc8c8fa32, 0xc0c0f030,
0xd7d721f6, 0x3232bc8e, 0xc6c675b3, 0x8f8f6fe0,
0x7474691d, 0xdbdb2ef5, 0x8b8b6ae1, 0xb8b8962e,
0x0a0a8a80, 0x9999fe67, 0x2b2be2c9, 0x8181e061,
0x0303c0c3, 0xa4a48d29, 0x8c8caf23, 0xaeae07a9,
0x3434390d, 0x4d4d1f52, 0x3939764f, 0xbdbdd36e,
0x575781d6, 0x6f6fb7d8, 0xdcdceb37, 0x15155144,
0x7b7ba6dd, 0xf7f709fe, 0x3a3ab68c, 0xbcbc932f,
0x0c0c0f03, 0xffff03fc, 0xa9a9c26b, 0xc9c9ba73,
0xb5b5d96c, 0xb1b1dc6d, 0x6d6d375a, 0x45451550,
0x3636b98f, 0x6c6c771b, 0xbebe13ad, 0x4a4ada90,
0xeeee57b9, 0x7777a9de, 0xf2f24cbe, 0xfdfd837e,
0x44445511, 0x6767bdda, 0x71712c5d, 0x05054540,
0x7c7c631f, 0x40405010, 0x6969325b, 0x6363b8db,
0x2828220a, 0x0707c5c2, 0xc4c4f531, 0x2222a88a,
0x969631a7, 0x3737f9ce, 0xeded977a, 0xf6f649bf,
0xb4b4992d, 0xd1d1a475, 0x434390d3, 0x48485a12,
0xe2e258ba, 0x979771e6, 0xd2d264b6, 0xc2c270b2,
0x2626ad8b, 0xa5a5cd68, 0x5e5ecb95, 0x2929624b,
0x30303c0c, 0x5a5ace94, 0xddddab76, 0xf9f9867f,
0x9595f164, 0xe6e65dbb, 0xc7c735f2, 0x24242d09,
0x1717d1c6, 0xb9b9d66f, 0x1b1bdec5, 0x12129486,
0x60607818, 0xc3c330f3, 0xf5f5897c, 0xb3b35cef,
0xe8e8d23a, 0x7373acdf, 0x3535794c, 0x8080a020,
0xe5e59d78, 0xbbbb56ed, 0x7d7d235e, 0xf8f8c63e,
0x5f5f8bd4, 0x2f2fe7c8, 0xe4e4dd39, 0x21216849,
];

static Sbox_T24: [u32;256] =
[
0xd55b5b8e, 0x924242d0, 0xeaa7a74d, 0xfdfbfb06,
0xcf3333fc, 0xe2878765, 0x3df4f4c9, 0xb5dede6b,
0x1658584e, 0xb4dada6e, 0x14505044, 0xc10b0bca,
0x28a0a088, 0xf8efef17, 0x2cb0b09c, 0x05141411,
0x2bacac87, 0x669d9dfb, 0x986a6af2, 0x77d9d9ae,
0x2aa8a882, 0xbcfafa46, 0x04101014, 0xc00f0fcf,
0xa8aaaa02, 0x45111154, 0x134c4c5f, 0x269898be,
0x4825256d, 0x841a1a9e, 0x0618181e, 0x9b6666fd,
0x9e7272ec, 0x4309094a, 0x51414110, 0xf7d3d324,
0x934646d5, 0xecbfbf53, 0x9a6262f8, 0x7be9e992,
0x33ccccff, 0x55515104, 0x0b2c2c27, 0x420d0d4f,
0xeeb7b759, 0xcc3f3ff3, 0xaeb2b21c, 0x638989ea,
0xe7939374, 0xb1cece7f, 0x1c70706c, 0xaba6a60d,
0xca2727ed, 0x08202028, 0xeba3a348, 0x975656c1,
0x82020280, 0xdc7f7fa3, 0x965252c4, 0xf9ebeb12,
0x74d5d5a1, 0x8d3e3eb3, 0x3ffcfcc3, 0xa49a9a3e,
0x461d1d5b, 0x071c1c1b, 0xa59e9e3b, 0xfff3f30c,
0xf0cfcf3f, 0x72cdcdbf, 0x175c5c4b, 0xb8eaea52,
0x810e0e8f, 0x5865653d, 0x3cf0f0cc, 0x1964647d,
0xe59b9b7e, 0x87161691, 0x4e3d3d73, 0xaaa2a208,
0x69a1a1c8, 0x6aadadc7, 0x83060685, 0xb0caca7a,
0x70c5c5b5, 0x659191f4, 0xd96b6bb2, 0x892e2ea7,
0xfbe3e318, 0xe8afaf47, 0x0f3c3c33, 0x4a2d2d67,
0x71c1c1b0, 0x5759590e, 0x9f7676e9, 0x35d4d4e1,
0x1e787866, 0x249090b4, 0x0e383836, 0x5f797926,
0x628d8def, 0x59616138, 0xd2474795, 0xa08a8a2a,
0x259494b1, 0x228888aa, 0x7df1f18c, 0x3bececd7,
0x01040405, 0x218484a5, 0x79e1e198, 0x851e1e9b,
0xd7535384, 0x00000000, 0x4719195e, 0x565d5d0b,
0x9d7e7ee3, 0xd04f4f9f, 0x279c9cbb, 0x5349491a,
0x4d31317c, 0x36d8d8ee, 0x0208080a, 0xe49f9f7b,
0xa2828220, 0xc71313d4, 0xcb2323e8, 0x9c7a7ae6,
0xe9abab42, 0xbdfefe43, 0x882a2aa2, 0xd14b4b9a,
0x41010140, 0xc41f1fdb, 0x38e0e0d8, 0xb7d6d661,
0xa18e8e2f, 0xf4dfdf2b, 0xf1cbcb3a, 0xcd3b3bf6,
0xfae7e71d, 0x608585e5, 0x15545441, 0xa3868625,
0xe3838360, 0xacbaba16, 0x5c757529, 0xa6929234,
0x996e6ef7, 0x34d0d0e4, 0x1a686872, 0x54555501,
0xafb6b619, 0x914e4edf, 0x32c8c8fa, 0x30c0c0f0,
0xf6d7d721, 0x8e3232bc, 0xb3c6c675, 0xe08f8f6f,
0x1d747469, 0xf5dbdb2e, 0xe18b8b6a, 0x2eb8b896,
0x800a0a8a, 0x679999fe, 0xc92b2be2, 0x618181e0,
0xc30303c0, 0x29a4a48d, 0x238c8caf, 0xa9aeae07,
0x0d343439, 0x524d4d1f, 0x4f393976, 0x6ebdbdd3,
0xd6575781, 0xd86f6fb7, 0x37dcdceb, 0x44151551,
0xdd7b7ba6, 0xfef7f709, 0x8c3a3ab6, 0x2fbcbc93,
0x030c0c0f, 0xfcffff03, 0x6ba9a9c2, 0x73c9c9ba,
0x6cb5b5d9, 0x6db1b1dc, 0x5a6d6d37, 0x50454515,
0x8f3636b9, 0x1b6c6c77, 0xadbebe13, 0x904a4ada,
0xb9eeee57, 0xde7777a9, 0xbef2f24c, 0x7efdfd83,
0x11444455, 0xda6767bd, 0x5d71712c, 0x40050545,
0x1f7c7c63, 0x10404050, 0x5b696932, 0xdb6363b8,
0x0a282822, 0xc20707c5, 0x31c4c4f5, 0x8a2222a8,
0xa7969631, 0xce3737f9, 0x7aeded97, 0xbff6f649,
0x2db4b499, 0x75d1d1a4, 0xd3434390, 0x1248485a,
0xbae2e258, 0xe6979771, 0xb6d2d264, 0xb2c2c270,
0x8b2626ad, 0x68a5a5cd, 0x955e5ecb, 0x4b292962,
0x0c30303c, 0x945a5ace, 0x76ddddab, 0x7ff9f986,
0x649595f1, 0xbbe6e65d, 0xf2c7c735, 0x0924242d,
0xc61717d1, 0x6fb9b9d6, 0xc51b1bde, 0x86121294,
0x18606078, 0xf3c3c330, 0x7cf5f589, 0xefb3b35c,
0x3ae8e8d2, 0xdf7373ac, 0x4c353579, 0x208080a0,
0x78e5e59d, 0xedbbbb56, 0x5e7d7d23, 0x3ef8f8c6,
0xd45f5f8b, 0xc82f2fe7, 0x39e4e4dd, 0x49212168,
];


macro_rules! SM4_T {
	($b:expr) => (Sbox_T[get_byte_u32($b,0) as usize] ^ (Sbox_T8[get_byte_u32($b,1) as usize]) ^ (Sbox_T16[get_byte_u32($b,2) as usize]) ^ (Sbox_T24[get_byte_u32($b,3) as usize]))
}

macro_rules! SM4_TP {
	($b:expr) => 
	{{
		let t: u32 = make_u32(Sbox[get_byte_u32($b,0) as usize],Sbox[get_byte_u32($b,1) as usize],Sbox[get_byte_u32($b,2) as usize],Sbox[get_byte_u32($b,3) as usize]);

		t ^ t.rotate_left(13) ^ t.rotate_left(23)
	}}
}

macro_rules! SM4_RNDS { 
	($k0:expr, $k1:expr, $k2:expr, $k3:expr, $B0:ident, $B1:ident, $B2:ident, $B3:ident, $Rk:ident) => (
	$B0 ^= SM4_T!($B1 ^ $B2 ^ $B3 ^ $Rk[$k0]);
  	$B1 ^= SM4_T!($B0 ^ $B2 ^ $B3 ^ $Rk[$k1]);
  	$B2 ^= SM4_T!($B0 ^ $B1 ^ $B3 ^ $Rk[$k2]);
  	$B3 ^= SM4_T!($B0 ^ $B1 ^ $B2 ^ $Rk[$k3]);
  	)
} 

pub struct SM4_Cryptor
{
	Mk: [u8;16],
	Rk: [u32;32],
	hasRkGenerated: bool,
}

pub fn getSm4Rk(Mk: &[u32;4]) -> [u32;32]
{
	let mut Rk = [0u32;32];

	let mut K: [u32;4] = [0;4];
	K[0] = &Mk[0] ^ FK[0];
	K[1] = &Mk[1] ^ FK[1];
	K[2] = &Mk[2] ^ FK[2];
	K[3] = &Mk[3] ^ FK[3];

	for i in 0..32
	{
		K[i%4] ^= SM4_TP!(K[(i+1)%4] ^ K[(i+2)%4] ^ K[(i+3)%4] ^ CK[i]);
		Rk[i] = K[i % 4];
	}

	Rk
}

pub fn sm4Enc(Rk: &[u32], ptxt: &[u32;4]) -> [u32;4]
{
		let mut B0: u32 = ptxt[0];
		let mut B1: u32 = ptxt[1];
		let mut B2: u32 = ptxt[2];
		let mut B3: u32 = ptxt[3];

		let mut ctxt: [u32;4] = [0;4];

		SM4_RNDS!( 0,  1,  2,  3, B0, B1, B2, B3, Rk);
		SM4_RNDS!( 4,  5,  6,  7, B0, B1, B2, B3, Rk);
		SM4_RNDS!( 8,  9, 10, 11, B0, B1, B2, B3, Rk);
		SM4_RNDS!(12, 13, 14, 15, B0, B1, B2, B3, Rk);
		SM4_RNDS!(16, 17, 18, 19, B0, B1, B2, B3, Rk);
		SM4_RNDS!(20, 21, 22, 23, B0, B1, B2, B3, Rk);
		SM4_RNDS!(24, 25, 26, 27, B0, B1, B2, B3, Rk);
		SM4_RNDS!(28, 29, 30, 31, B0, B1, B2, B3, Rk);

		ctxt[0] = B3;
		ctxt[1] = B2;
		ctxt[2] = B1;
		ctxt[3] = B0;

		ctxt
}

pub fn sm4Dec(Rk: &[u32], ctxt: &[u32;4]) -> [u32;4]
{
		let mut B0: u32 = ctxt[0];
		let mut B1: u32 = ctxt[1];
		let mut B2: u32 = ctxt[2];
		let mut B3: u32 = ctxt[3];

		let mut ptxt:[u32;4] = [0;4];

		SM4_RNDS!(31, 30, 29, 28, B0, B1, B2, B3, Rk);
		SM4_RNDS!(27, 26, 25, 24, B0, B1, B2, B3, Rk);
		SM4_RNDS!(23, 22, 21, 20, B0, B1, B2, B3, Rk);
		SM4_RNDS!(19, 18, 17, 16, B0, B1, B2, B3, Rk);
		SM4_RNDS!(15, 14, 13, 12, B0, B1, B2, B3, Rk);
		SM4_RNDS!(11, 10,  9,  8, B0, B1, B2, B3, Rk);
		SM4_RNDS!( 7,  6,  5,  4, B0, B1, B2, B3, Rk);
		SM4_RNDS!( 3,  2,  1,  0, B0, B1, B2, B3, Rk);

		ptxt[0] = B3;
		ptxt[1] = B2;
		ptxt[2] = B1;
		ptxt[3] = B0;

		ptxt
}