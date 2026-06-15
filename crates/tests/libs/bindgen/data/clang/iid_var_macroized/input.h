// Test: IID extracted from `const GUID IID_XXX = { ... }` variable declarations
// and applied to interfaces that lack __declspec(uuid(...)).

typedef struct _GUID {
    unsigned int Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID, IID;

struct IUnknown {
    virtual int __stdcall QueryInterface(void* riid, void** ppvObject) = 0;
    virtual unsigned int __stdcall AddRef() = 0;
    virtual unsigned int __stdcall Release() = 0;
};

#define HRESULT unsigned long
#define UInt32 unsigned int

#define STDMETHODIMP                HRESULT __stdcall
#define DECLSPEC_NOTHROW            __declspec(nothrow)
#define COM_DECLSPEC_NOTHROW        DECLSPEC_NOTHROW

#define Z7_COMWF_B                  COM_DECLSPEC_NOTHROW STDMETHODIMP
#define Z7_COM7F_B                  Z7_COMWF_B
#define Z7_COM7F_E                  throw()
#define Z7_COM7F_IMF(f)             Z7_COM7F_B f Z7_COM7F_E
#define Z7_COM7F_PURE(f)            virtual Z7_COM7F_IMF(f) =0;
#define Z7_IFACE_PURE(name)         Z7_IFACEN_ ## name(=0;)
#define Z7_IFACE_COM7_PURE(name)    Z7_IFACEM_ ## name(Z7_COM7F_PURE)

#define k_7zip_GUID_Data1 0x23170F69
#define k_7zip_GUID_Data2 0x40C1
#define k_7zip_GUID_Data3_Common  0x278A

#define Z7_DEFINE_GUID(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8) \
    extern "C" const GUID name; \
    extern "C" const GUID name = { l, w1, w2, { b1, b2, b3, b4, b5, b6, b7, b8 } }

#define Z7_DECLSPEC_NOVTABLE __declspec(novtable)

#define Z7_DECL_IFACE_7ZIP_SUB(i, _base, groupId, subId) \
    Z7_DEFINE_GUID(IID_ ## i, \
    k_7zip_GUID_Data1, \
    k_7zip_GUID_Data2, \
    k_7zip_GUID_Data3_Common, \
    0, 0, 0, (groupId), 0, (subId), 0, 0); \
    //struct Z7_DECLSPEC_NOVTABLE i: public _base

#define Z7_IFACE_CONSTR_STREAM_SUB(i, base, n) \
    Z7_DECL_IFACE_7ZIP_SUB(i, base, 3, n) \
    //{ Z7_IFACE_COM7_PURE(i) };

#define Z7_IFACE_CONSTR_STREAM(i, n) \
    Z7_IFACE_CONSTR_STREAM_SUB(i, IUnknown, n)

#define Z7_IFACEM_ISequentialInStream(x) \
    x(Read(void *data, UInt32 size, UInt32 *processedSize))
Z7_IFACE_CONSTR_STREAM(ISequentialInStream, 0x01)

//atm interfaces built via macro expansion dont work without adding extra clang filters,
// but not testing that here, so commented out and direct expansion here instead.
struct ISequentialInStream: public IUnknown { 
    virtual unsigned long __stdcall Read(void *data, unsigned int size, unsigned int *processedSize) =0; 
};