// bindgen-flags: --with-derive-hash --with-derive-partialeq
//
union pixel {
    unsigned int rgba;
    struct {
        unsigned char r;
        unsigned char g;
        unsigned char b;
        unsigned char a;
    };
};
