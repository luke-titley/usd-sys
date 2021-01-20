#include "bind_defs.hpp"

#include "pxr/base/tf/token.h"
#include "pxr/usd/usd/prim.h"

namespace cppmm_bind {

namespace PXR_INTERNAL_NS {

namespace pxr = ::PXR_INTERNAL_NS;

class UsdPrim {

    const pxr::TfToken &GetTypeName() const;
    const pxr::TfToken &GetName() const;

} CPPMM_OPAQUEBYTES;

}
}