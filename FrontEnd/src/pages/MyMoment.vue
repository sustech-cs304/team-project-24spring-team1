<template>
    <div>
        <div class="row">
            <div class="col-lg-4" :class="{ 'text-right': isRTL }">
                <card style="width: 50rem;">
                    <img slot="image" class="card-img-top" src="data:image/svg+xml;charset=UTF-8,%3Csvg%20width%3D%22320%22%20height%3D%22180%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%20320%20180%22%20preserveAspectRatio%3D%22none%22%3E%3Cdefs%3E%3Cstyle%20type%3D%22text%2Fcss%22%3E%23holder_1664f6b99d2%20text%20%7B%20fill%3Argba(255%2C255%2C255%2C.75)%3Bfont-weight%3Anormal%3Bfont-family%3AHelvetica%2C%20monospace%3Bfont-size%3A16pt%20%7D%20%3C%2Fstyle%3E%3C%2Fdefs%3E%3Cg%20id%3D%22holder_1664f6b99d2%22%3E%3Crect%20width%3D%22320%22%20height%3D%22180%22%20fill%3D%22%23777%22%3E%3C%2Frect%3E%3Cg%3E%3Ctext%20x%3D%22119.0859375%22%20y%3D%2297.35%22%3E320x180%3C%2Ftext%3E%3C%2Fg%3E%3C%2Fg%3E%3C%2Fsvg%3E" alt="Card image cap"/>
                    <p class="card-text">Content</p>
                    <div class="d-flex justify-content-around">
                        <base-button round icon type="primary">
                            <i class="tim-icons icon-heart-2"></i>
                        </base-button>
                        <base-button round icon type="primary">
                            <i class="tim-icons icon-chat-33"></i>
                        </base-button>
                        <base-button round icon type="primary">
                            <i class="tim-icons icon-simple-delete"></i>
                        </base-button>
                    </div>
                    <div class="dropdown-divider"></div>
                    <div>
                        <card class="mb-3">
                            <h4 class="card-title">User 1</h4>
                            <p class="card-text">This is a wider card with supporting text below as a natural lead-in to additional content. This content is a little bit longer.</p>
                            <p class="card-text"><small class="text-muted">Last updated 1 mins ago</small></p>
                        </card>
                    </div>
                    <div class="dropdown-divider"></div>
                    <div>
                        <card class="mb-3">
                            <h4 class="card-title">User 2</h4>
                            <p class="card-text">This is a wider card with supporting text below as a natural lead-in to additional content. This content is a little bit longer.</p>
                            <p class="card-text"><small class="text-muted">Last updated 10 mins ago</small></p>
                        </card>
                    </div>
                </card>
                <card style="width: 50rem;">
                    <img slot="image" class="card-img-top" src="data:image/svg+xml;charset=UTF-8,%3Csvg%20width%3D%22320%22%20height%3D%22180%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%20320%20180%22%20preserveAspectRatio%3D%22none%22%3E%3Cdefs%3E%3Cstyle%20type%3D%22text%2Fcss%22%3E%23holder_1664f6b99d2%20text%20%7B%20fill%3Argba(255%2C255%2C255%2C.75)%3Bfont-weight%3Anormal%3Bfont-family%3AHelvetica%2C%20monospace%3Bfont-size%3A16pt%20%7D%20%3C%2Fstyle%3E%3C%2Fdefs%3E%3Cg%20id%3D%22holder_1664f6b99d2%22%3E%3Crect%20width%3D%22320%22%20height%3D%22180%22%20fill%3D%22%23777%22%3E%3C%2Frect%3E%3Cg%3E%3Ctext%20x%3D%22119.0859375%22%20y%3D%2297.35%22%3E320x180%3C%2Ftext%3E%3C%2Fg%3E%3C%2Fg%3E%3C%2Fsvg%3E" alt="Card image cap"/>
                    <p class="card-text">Content</p>
                    <div class="d-flex justify-content-around">
                        <base-button round icon type="primary">
                            <i class="tim-icons icon-heart-2"></i>
                        </base-button>
                        <base-button round icon type="primary">
                            <i class="tim-icons icon-chat-33"></i>
                        </base-button>
                        <base-button round icon type="primary">
                            <i class="tim-icons icon-simple-delete"></i>
                        </base-button>
                    </div>
                    <div class="dropdown-divider"></div>
                    <div>
                        <card class="mb-3">
                            <h4 class="card-title">User 1</h4>
                            <p class="card-text">This is a wider card with supporting text below as a natural lead-in to additional content. This content is a little bit longer.</p>
                            <p class="card-text"><small class="text-muted">Last updated 1 mins ago</small></p>
                        </card>
                    </div>
                    <div class="dropdown-divider"></div>
                    <div>
                        <card class="mb-3">
                            <h4 class="card-title">User 2</h4>
                            <p class="card-text">This is a wider card with supporting text below as a natural lead-in to additional content. This content is a little bit longer.</p>
                            <p class="card-text"><small class="text-muted">Last updated 10 mins ago</small></p>
                        </card>
                    </div>
                </card>
            </div>
        </div>
    </div>
</template>