#pragma once

#include "amm_host.h"

struct ID3D11Device;
struct ID3D11DeviceContext;

namespace amm_shim {

// draws the layers of one AmmHostFrame with D3D11, bottom to top, in one pass. Render thread only. Works on the
// game's own D3D11 device and on the D3D11On12 device of the D3D12 path; the caller binds the render target and
// keeps the context state it needs. Each draw command picks its pixel shader by `shader` and its sampler by
// `sampler`; a command with an unknown shader or sampler is skipped and logged once
class LayerRenderer {
public:
    // makes the fixed pipeline objects on `device`; false leaves nothing behind. The shaders compile once per
    // process, on the first call; later calls build only device objects from the kept bytecode
    static bool init(ID3D11Device* device);

    // true between a successful init() and release()
    static bool ready();

    // drops the freed owners, applies every layer's texture deltas, draws every layer into the bound target of
    // `width` x `height` pixels, then frees the textures each layer lists
    static void draw(ID3D11DeviceContext* ctx, ID3D11Device* device, AmmHostFrame const& frame, float width, float height);

    // re-arms the once-per-open logs of the manager layer; called for a frame that has no manager layer
    static void managerClosed();

    // drops every device object and every layer texture; the compiled bytecode stays
    static void release();
};

} // namespace amm_shim
